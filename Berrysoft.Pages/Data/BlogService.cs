using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;

namespace Berrysoft.Pages.Data
{
    public enum BlogPostType
    {
        Text,
        Html,
        Markdown
    }

    public class BlogPost
    {
        public string Title { get; set; }
        public DateTime Date { get; set; }
        public string Filename { get; set; }
        [JsonConverter(typeof(JsonStringEnumConverter))]
        public BlogPostType Type { get; set; }
    }

    public interface IBlogService : IDataLoaderService<IEnumerable<BlogPost>>
    {
        ValueTask<BlogPost> GetBlogPostAsync(string filename);
        ValueTask<string> GetBlogPostContentAsync(string filename);
    }

    public class BlogService : IBlogService
    {
        protected HttpClient Http { get; set; }
        public BlogService(HttpClient http) => Http = http;

        public IEnumerable<BlogPost> Data { get; private set; }
        private static readonly SemaphoreLocker blogsLocker = new SemaphoreLocker();

        public ValueTask LoadDataAsync()
        {
            if (Data == null)
            {
                return blogsLocker.LockAsync(async () =>
                {
                    if (Data == null)
                    {
                        Data = await Http.GetJsonAsync<BlogPost[]>("blog/index.json");
                    }
                });
            }
            else
            {
                return new ValueTask();
            }
        }

        public async ValueTask<BlogPost> GetBlogPostAsync(string filename)
        {
            await LoadDataAsync();
            return Data.Where(post => post.Filename == filename).FirstOrDefault();
        }

        private string GetExtensionFromType(BlogPostType type)
        {
            return type switch
            {
                BlogPostType.Text => "txt",
                BlogPostType.Html => "htm",
                BlogPostType.Markdown => "md",
                _ => string.Empty
            };
        }

        public async ValueTask<string> GetBlogPostContentAsync(string filename)
        {
            var post = await GetBlogPostAsync(filename);
            if (post != null)
            {
                var url = $"blog/{filename}.{GetExtensionFromType(post.Type)}";
                return await Http.GetStringAsync(url);
            }
            else
            {
                return null;
            }
        }
    }
}
