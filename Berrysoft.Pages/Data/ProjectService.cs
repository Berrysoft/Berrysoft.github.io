using System.Net.Http;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;

namespace Berrysoft.Pages.Data
{
    public class ProjectService : IProjectService
    {
        protected HttpClient Http { get; set; }

        public ProjectService(HttpClient http) => Http = http;

        private ProjectBox[] projects;

        public async Task<ProjectBox[]> GetProjectsAsync()
        {
            if (projects == null)
            {
                projects = await Http.GetJsonAsync<ProjectBox[]>("data/projects.json");
            }
            return projects;
        }
    }
}
