using Berrysoft.Pages.Data;
using Berrysoft.Pages.HighlightJs;
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
            services.AddSingleton<ICounterService, CounterService>();
            services.AddSingleton<IBlogService, BlogService>();
            services.AddSingleton<IHighlightJsEngine, HighlightJsEngine>();
            services.AddSingleton<LibraryService>();
        }

        public void Configure(IComponentsApplicationBuilder app)
        {
            app.UseLoadingBar();
            app.AddComponent<App>("app");
        }
    }
}
