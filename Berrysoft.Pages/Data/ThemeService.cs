using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;
using Microsoft.JSInterop;

namespace Berrysoft.Pages.Data
{
    public enum ThemeType
    {
        Light,
        Dark
    }

    public class Theme
    {
        public string Name { get; set; }
        [JsonConverter(typeof(JsonStringEnumConverter))]
        public ThemeType Navbar { get; set; }
        public Dictionary<string, string> Links { get; set; }
    }

    public delegate void ThemeChangedCallback(object e, Theme t);

    public interface IThemeService : IDataLoaderService<IEnumerable<string>>
    {
        string Theme { get; set; }
        ThemeType Navbar { get; }
        event ThemeChangedCallback ThemeChanged;
    }

    public class ThemeService : IThemeService
    {
        protected HttpClient Http { get; set; }
        protected IJSRuntime JSRuntime { get; set; }

        public ThemeService(HttpClient http, IJSRuntime jSRuntime)
        {
            Http = http;
            JSRuntime = jSRuntime;
        }

        private Theme[] themes;
        public IEnumerable<string> Data => themes?.Select(t => t.Name);
        private static readonly SemaphoreLocker themesLocker = new SemaphoreLocker();

        public ThemeType Navbar { get; private set; }

        private string theme;
        public string Theme
        {
            get => theme;
            set => SetTheme(value);
        }
        private async void SetTheme(string value) => await SetThemeAsync(value);

        private async ValueTask SetThemeAsync(string value)
        {
            await LoadDataAsync();
            if (theme != value && !string.IsNullOrEmpty(value))
            {
                var storedTheme = themes.Where(t => t.Name == value).FirstOrDefault();
                await SetThemeAsync(storedTheme);
            }
        }

        public event ThemeChangedCallback ThemeChanged;

        private async ValueTask SetThemeAsync(Theme storedTheme)
        {
            if (storedTheme != null)
            {
                theme = storedTheme.Name;
                Navbar = storedTheme.Navbar;
                foreach (var pair in storedTheme.Links)
                {
                    await JSRuntime.InvokeAsync<object>("changeStyle", pair.Key, pair.Value);
                }
                ThemeChanged?.Invoke(this, storedTheme);
            }
        }

        public ValueTask LoadDataAsync()
        {
            if (themes == null)
            {
                return themesLocker.LockAsync(async () =>
                {
                    if (themes == null)
                    {
                        themes = await Http.GetJsonAsync<Theme[]>("css/index.json");
                        var defaultTheme = themes.FirstOrDefault();
                        await SetThemeAsync(defaultTheme);
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
