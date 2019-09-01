using System.Threading.Tasks;

namespace Berrysoft.Pages.Data
{
    public interface IProjectService
    {
        Task<ProjectBox[]> GetProjectsAsync();
    }
}
