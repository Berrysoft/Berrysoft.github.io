using System.Net.Http;

namespace Berrysoft.Pages.Data
{
    public class FriendLink
    {
        public string? Name { get; set; }
        public string? Title { get; set; }
        public string? Url { get; set; }
    }

    public class LinkService : EnumerableLoaderService<FriendLink>
    {
        public LinkService(HttpClient http) : base("data/links.json", http) { }
    }
}
