using System.Net.Http;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;

namespace Berrysoft.Pages.Data
{
    public class ProjectBox
    {
        public string Name { get; set; }
        public string Url { get; set; }
        public string Language { get; set; }
        public string Description { get; set; }
    }

    public interface IProjectService
    {
        ValueTask<ProjectBox[]> GetProjectsAsync();
    }

    public class ProjectService : IProjectService
    {
        protected HttpClient Http { get; set; }

        public ProjectService(HttpClient http) => Http = http;

        private ProjectBox[] projects;
        private static readonly SemaphoreLocker projectsLocker = new SemaphoreLocker();

        private ValueTask InitializeProjects()
        {
            if (projects == null)
            {
                return projectsLocker.LockAsync(async () =>
                {
                    if (projects == null)
                    {
                        projects = await Http.GetJsonAsync<ProjectBox[]>("data/projects.json");
                    }
                });
            }
            else
            {
                return new ValueTask();
            }
        }

        public async ValueTask<ProjectBox[]> GetProjectsAsync()
        {
            await InitializeProjects();
            return projects;
        }
    }
}
