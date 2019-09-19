using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Text.Json.Serialization;

namespace Berrysoft.Pages.Data
{
    public class GitHubRepo
    {
        public int Id { get; set; }
        public string? Name { get; set; }
    }

    public class GitHubCommit
    {
        public string? Sha { get; set; }
        public string? Message { get; set; }
    }

    public class GitHubPushPayload
    {
        public IEnumerable<GitHubCommit>? Commits { get; set; }
    }

    public class GitHubEvent
    {
        public string? Type { get; set; }
        public GitHubRepo? Repo { get; set; }
        public GitHubPushPayload? Payload { get; set; }
        [JsonPropertyName("created_at")]
        public DateTime CreatedAt { get; set; }
    }

    public class GitHubEventService : EnumerableLoaderService<GitHubEvent>
    {
        public GitHubEventService(HttpClient http) : base("https://api.github.com/users/berrysoft/events", http) { }
    }
}
