using System.Threading.Tasks;
using Microsoft.JSInterop;

namespace Berrysoft.Pages.Data
{
    public interface ILocalStorage
    {
        ValueTask<bool> GetStorageAvaliableAsync();
        ValueTask SetItemAsync(string key, string value);
        ValueTask<string> GetItemAsync(string key);
        ValueTask<bool> ContainsKeyAsync(string key);
        ValueTask RemoveAsync(string key);
    }

    public class LocalStorage : ILocalStorage
    {
        protected IJSRuntime JSRuntime { get; set; }

        public LocalStorage(IJSRuntime jSRuntime) => JSRuntime = jSRuntime;

        public ValueTask<bool> GetStorageAvaliableAsync() => JSRuntime.InvokeAsync<bool>("storageAvailable", "localStorage");

        public ValueTask SetItemAsync(string key, string value) => JSRuntime.InvokeVoidAsync("localStorageSetItem", key, value);

        public ValueTask<string> GetItemAsync(string key) => JSRuntime.InvokeAsync<string>("localStorageGetItem", key);

        public ValueTask<bool> ContainsKeyAsync(string key) => JSRuntime.InvokeAsync<bool>("localStorageHasKey", key);

        public ValueTask RemoveAsync(string key) => JSRuntime.InvokeVoidAsync("localStorageRemoveItem", key);
    }
}
