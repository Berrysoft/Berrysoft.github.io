﻿using System;
using System.Net.Http;
using System.Threading.Tasks;
using Berrysoft.Pages.Data;
using Berrysoft.Pages.HighlightJs;
using Berrysoft.Pages.Katex;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Microsoft.Extensions.DependencyInjection;
using Toolbelt.Blazor.Extensions.DependencyInjection;

namespace Berrysoft.Pages
{
    public class Program
    {
        public static Task Main()
        {
            var builder = WebAssemblyHostBuilder.CreateDefault();
            builder.RootComponents.Add<App>("app");
            builder.Services.AddSingleton(new HttpClient { BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) });
            builder.Services.AddLoadingBar();
            builder.Services.AddSingleton<ILocalStorage, LocalStorage>();
            builder.Services.AddSingleton<LocalizationService>();
            builder.Services.AddSingleton<ThemeService>();
            builder.Services.AddSingleton<ProjectService>();
            builder.Services.AddSingleton<GitHubEventService>();
            builder.Services.AddSingleton<LinkService>();
            builder.Services.AddSingleton<CounterService>();
            builder.Services.AddSingleton<TeaService>();
            builder.Services.AddSingleton<BlogService>();
            builder.Services.AddHighlightJs();
            builder.Services.AddKatex();
            builder.Services.AddSingleton<LibraryService>();
            return builder.Build().UseLoadingBar().RunAsync();
        }
    }
}
