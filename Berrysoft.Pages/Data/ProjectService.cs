using System.Collections.Generic;
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

    public interface IProjectService : IDataLoaderService<IEnumerable<ProjectBox>> { }

    public class ProjectService : IProjectService
    {
        protected HttpClient Http { get; set; }

        public ProjectService(HttpClient http)
        {
            Http = http;
            LoadData();
        }

        public IEnumerable<ProjectBox> Data { get; private set; }

        private static readonly SemaphoreLocker projectsLocker = new SemaphoreLocker();

        private async void LoadData() => await LoadDataAsync();

        public ValueTask LoadDataAsync()
        {
            if (Data == null)
            {
                return projectsLocker.LockAsync(async () =>
                {
                    if (Data == null)
                    {
                        Data = await Http.GetJsonAsync<ProjectBox[]>("data/projects.json");
                    }
                });
            }
            else
            {
                return new ValueTask();
            }
        }
    }
}
