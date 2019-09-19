using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Threading.Tasks;
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
        public string? Filename { get; set; }
    }

    public interface IBlogService : IDataLoaderService<IEnumerable<BlogPost>?>
    {
        ValueTask<(BlogPost?, string?)> GetBlogPostAsync(string filename);
    }

    public class BlogService : EnumerableLoaderService<BlogPost>, IBlogService
    {
        protected IHighlightJsEngine HighlightJsEngine { get; set; }

        protected IKatexEngine KatexEngine { get; set; }

        protected MarkdownPipeline MarkdigPipeline { get; set; }

        public BlogService(HttpClient http, IHighlightJsEngine highlightJsEngine, IKatexEngine katexEngine) : base("blogdata/index.json", http)
        {
            HighlightJsEngine = highlightJsEngine;
            KatexEngine = katexEngine;
            MarkdigPipeline = new MarkdownPipelineBuilder().UseAdvancedExtensions().UseHighlightJs(HighlightJsEngine).UseKatex(KatexEngine).Build();
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
