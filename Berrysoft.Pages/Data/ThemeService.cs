using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;
using Microsoft.JSInterop;

namespace Berrysoft.Pages.Data
{
    public interface IThemeService
    {
        Task<string[]> GetThemesAsync();
        string Theme { get; set; }
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

        private Dictionary<string, Dictionary<string, string>> themes;
        private static readonly SemaphoreLocker themesLocker = new SemaphoreLocker();

        private string theme;
        public string Theme
        {
            get => theme;
            set => SetTheme(value);
        }
        private async void SetTheme(string value) => await SetThemeAsync(value);

        private async Task SetThemeAsync(string value)
        {
            if (theme != value && !string.IsNullOrEmpty(value) && themes.ContainsKey(value))
            {
                theme = value;
                foreach (var pair in themes[value])
                {
                    await JSRuntime.InvokeAsync<object>("changeStyle", pair.Key, pair.Value);
                }
            }
        }

        private Task InitializeThemes()
        {
            if (themes == null)
            {
                return themesLocker.LockAsync(async () =>
                {
                    if (themes == null)
                    {
                        themes = await Http.GetJsonAsync<Dictionary<string, Dictionary<string, string>>>("css/index.json");
                        string defaultTheme = themes.Keys.FirstOrDefault();
                        if (defaultTheme != null)
                            await SetThemeAsync(defaultTheme);
                    }
                });
            }
            else
            {
                return Task.CompletedTask;
            }
        }

        public async Task<string[]> GetThemesAsync()
        {
            await InitializeThemes();
            return themes.Keys.ToArray();
        }
    }
}
