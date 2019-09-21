using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.ServiceModel.Syndication;
using System.Threading.Tasks;
using System.Xml;
using Berrysoft.Pages.HighlightJs;
using Berrysoft.Pages.Katex;
using Markdig;
using Pek.Markdig.HighlightJs;

namespace Berrysoft.Pages.Data
{
    public class BlogPost
    {
        public string? Title { get; set; }
        public DateTime Date { get; set; }
        public string? Description { get; set; }
        public string? Filename { get; set; }
    }

    public class BlogService
    {
        protected HttpClient Http { get; set; }

        protected IHighlightJsEngine HighlightJsEngine { get; set; }

        protected IKatexEngine KatexEngine { get; set; }

        protected MarkdownPipeline MarkdigPipeline { get; set; }

        public BlogService(HttpClient http, IHighlightJsEngine highlightJsEngine, IKatexEngine katexEngine)
        {
            Http = http;
            HighlightJsEngine = highlightJsEngine;
            KatexEngine = katexEngine;
            MarkdigPipeline = new MarkdownPipelineBuilder().UseAdvancedExtensions().UseHighlightJs(HighlightJsEngine).UseKatex(KatexEngine).Build();
        }

        public IEnumerable<BlogPost>? Data { get; private set; }
        private static readonly SemaphoreLocker blogsLocker = new SemaphoreLocker();

        public ValueTask LoadDataAsync()
        {
            if (Data == null)
            {
                return blogsLocker.LockAsync(async () =>
                {
                    if (Data == null)
                    {
                        using var response = await Http.GetAsync("blogdata/rss.xml");
                        using var stream = await response.Content.ReadAsStreamAsync();
                        using var reader = XmlReader.Create(stream);
                        var feed = SyndicationFeed.Load(reader);
                        Data = feed.Items.Select(item => new BlogPost
                        {
                            Title = item.Title?.Text,
                            Description = item.Summary?.Text,
                            Date = item.LastUpdatedTime.LocalDateTime,
                            Filename = item.Links?.FirstOrDefault()?.Uri?.LocalPath?.Split('/')?.LastOrDefault()
                        });
                    }
                });
            }
            else
            {
                return new ValueTask();
            }
        }

        public async ValueTask<(BlogPost?, string?)> GetBlogPostAsync(string filename)
        {
            await LoadDataAsync();
            var post = Data!.Where(post => post.Filename == filename).FirstOrDefault();
            return (post, await GetBlogPostContentAsync(post));
        }

        private async ValueTask<string?> GetBlogPostContentAsync(BlogPost? post)
        {
            if (post != null)
            {
                var url = $"blogdata/{post.Filename}.md";
                string md = await Http.GetStringAsync(url);
                return Markdown.ToHtml(md, MarkdigPipeline);
            }
            else
            {
                return null;
            }
        }
    }
}
