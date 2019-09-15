using System.Collections.Generic;
using System.Net.Http;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;

namespace Berrysoft.Pages.Data
{
    public interface IDataLoaderService<T>
    {
        ValueTask LoadDataAsync();
        T Data { get; }
    }

    public class EnumerableLoaderService<T> : IDataLoaderService<IEnumerable<T>?>
    {
        protected string Uri { get; set; }
        protected HttpClient Http { get; set; }

        public EnumerableLoaderService(string uri, HttpClient http)
        {
            Uri = uri;
            Http = http;
        }

        public IEnumerable<T>? Data { get; private set; }
        private static readonly SemaphoreLocker dataLocker = new SemaphoreLocker();

        public ValueTask LoadDataAsync()
        {
            if (Data == null)
            {
                return dataLocker.LockAsync(async () =>
                {
                    if (Data == null)
                    {
                        Data = await Http.GetJsonAsync<T[]>(Uri);
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
