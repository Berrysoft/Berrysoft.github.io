using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;

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
        ValueTask<BlogPost?> GetBlogPostAsync(string filename);
        ValueTask<string?> GetBlogPostContentAsync(string filename);
    }

    public class BlogService : IBlogService
    {
        protected HttpClient Http { get; set; }
        public BlogService(HttpClient http) => Http = http;

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
                        Data = await Http.GetJsonAsync<BlogPost[]>("blogdata/index.json");
                    }
                });
            }
            else
            {
                return new ValueTask();
            }
        }

        public async ValueTask<BlogPost?> GetBlogPostAsync(string filename)
        {
            await LoadDataAsync();
            return Data!.Where(post => post.Filename == filename).FirstOrDefault();
        }

        public async ValueTask<string?> GetBlogPostContentAsync(string filename)
        {
            var post = await GetBlogPostAsync(filename);
            if (post != null)
            {
                var url = $"blogdata/{filename}.md";
                return await Http.GetStringAsync(url);
            }
            else
            {
                return null;
            }
        }
    }
}
