using System.Net.Http;

namespace Berrysoft.Pages.Data
{
    public class LibraryBox
    {
        public string? Name { get; set; }
        public string? Url { get; set; }
        public string? License { get; set; }
        public string? LicenseUrl { get; set; }
    }

    public class LibraryService : EnumerableLoaderService<LibraryBox>
    {
        public LibraryService(HttpClient http) : base("data/libraries.json", http) { }
    }
}
