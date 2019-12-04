using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
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
        public string? Name { get; set; }
        // Avoid cannot find converter bug.
        [JsonPropertyName("navbar")]
        public string? NavbarInternal { get; set; }
        [JsonIgnore]
        public ThemeType Navbar => NavbarInternal!.ToLower() switch
        {
            "light" => ThemeType.Light,
            "dark" => ThemeType.Dark,
            _ => ThemeType.Light
        };
        public Dictionary<string, string>? Links { get; set; }
    }

    public delegate void ThemeChangedCallback(object e, Theme t);
    public delegate ValueTask ThemeChangedAsyncCallback(object e, Theme t);

    public class ThemeService : EnumerableLoaderService<Theme>
    {
        protected IJSRuntime JSRuntime { get; set; }

        public ThemeService(HttpClient http, IJSRuntime jSRuntime)
            : base("css/index.json", http)
        {
            JSRuntime = jSRuntime;
        }

        public ThemeType Navbar { get; private set; }

        private string? theme;
        public string? Theme
        {
            get => theme;
            set => SetTheme(value);
        }
        private async void SetTheme(string? value) => await SetThemeAsync(value);

        public async ValueTask SetThemeAsync(string? value)
        {
            await LoadDataAsync();
            if (theme != value && !string.IsNullOrEmpty(value))
            {
                Theme? storedTheme = Data.Where(t => t.Name == value).FirstOrDefault();
                await SetThemeAsync(storedTheme);
            }
        }

        public event ThemeChangedCallback? ThemeChanged;
        public event ThemeChangedAsyncCallback? ThemeChangedAsync;

        protected virtual async ValueTask OnThemeChangedAsync(Theme t)
        {
            if (ThemeChangedAsync != null)
                await ThemeChangedAsync(this, t);
            ThemeChanged?.Invoke(this, t);
        }

        private async ValueTask SetThemeAsync(Theme? storedTheme)
        {
            if (storedTheme != null)
            {
                theme = storedTheme.Name;
                Navbar = storedTheme.Navbar;
                if (storedTheme.Links != null)
                {
                    foreach (var pair in storedTheme.Links)
                    {
                        await JSRuntime.InvokeVoidAsync("changeStyle", pair.Key, pair.Value);
                    }
                }
                await OnThemeChangedAsync(storedTheme);
            }
        }
    }
}
