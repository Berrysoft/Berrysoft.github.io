using Markdig;
using Microsoft.Extensions.DependencyInjection;

namespace Berrysoft.Pages.Katex
{
    public static class KatexExtensions
    {
        public static MarkdownPipelineBuilder UseKatex(this MarkdownPipelineBuilder pipeline, IKatexEngine katexEngine)
        {
            pipeline.Extensions.Add(new KatexExtension(katexEngine));
            return pipeline;
        }

        public static IServiceCollection AddKatex(this IServiceCollection services)
        {
            return services.AddSingleton<IKatexEngine, KatexEngine>();
        }
    }
}
