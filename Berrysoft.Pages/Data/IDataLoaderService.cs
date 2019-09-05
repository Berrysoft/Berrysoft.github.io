using System.Threading.Tasks;

namespace Berrysoft.Pages.Data
{
    public interface IDataLoaderService<T>
    {
        ValueTask LoadDataAsync();
        T Data { get; }
    }
}
