using Markdig;
using Microsoft.Extensions.DependencyInjection;
using Pek.Markdig.HighlightJs;

namespace Berrysoft.Pages.HighlightJs
{
    public static class HighlightJsExtensions
    {
        public static MarkdownPipelineBuilder UseHighlightJs(this MarkdownPipelineBuilder pipeline, IHighlightJsEngine highlightJsEngine)
        {
            pipeline.Extensions.Add(new HighlightJsExtension(highlightJsEngine));
            return pipeline;
        }

        public static IServiceCollection AddHighlightJs(this IServiceCollection services)
        {
            return services.AddSingleton<IHighlightJsEngine, HighlightJsEngine>();
        }
    }
}
