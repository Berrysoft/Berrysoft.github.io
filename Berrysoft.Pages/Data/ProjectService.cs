﻿using System.Net.Http;

namespace Berrysoft.Pages.Data
{
    public class ProjectBox
    {
        public string? Name { get; set; }
        public string? Url { get; set; }
        public string? Language { get; set; }
        public string? Description { get; set; }
    }

    public class ProjectService : EnumerableLoaderService<ProjectBox>
    {
        public ProjectService(HttpClient http) : base("data/projects.json", http) { }
    }
}
