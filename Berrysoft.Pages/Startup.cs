using Berrysoft.Pages.Data;
using Berrysoft.Pages.HighlightJs;
using Berrysoft.Pages.Katex;
using Microsoft.AspNetCore.Components.Builder;
using Microsoft.Extensions.DependencyInjection;
using Toolbelt.Blazor.Extensions.DependencyInjection;

namespace Berrysoft.Pages
{
    public class Startup
    {
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddLoadingBar();
            services.AddSingleton<ILocalStorage, LocalStorage>();
            services.AddSingleton<ILocalizationService, LocalizationService>();
            services.AddSingleton<IThemeService, ThemeService>();
            services.AddSingleton<ProjectService>();
            services.AddSingleton<GitHubEventService>();
            services.AddSingleton<LinkService>();
            services.AddSingleton<ICounterService, CounterService>();
            services.AddSingleton<IBlogService, BlogService>();
            services.AddHighlightJs();
            services.AddKatex();
            services.AddSingleton<LibraryService>();
        }

        public void Configure(IComponentsApplicationBuilder app)
        {
            app.UseLoadingBar();
            app.UseLocalTimeZone();
            app.AddComponent<App>("app");
        }
    }
}
