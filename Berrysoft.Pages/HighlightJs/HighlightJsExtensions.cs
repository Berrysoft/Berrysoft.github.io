using Markdig;

namespace Berrysoft.Pages.HighlightJs
{
    public static class HighlightJsExtensions
    {
        public static MarkdownPipelineBuilder UseHighlightJs(this MarkdownPipelineBuilder pipeline, IHighlightJsEngine highlightJsEngine)
        {
            pipeline.Extensions.Add(new HighlightJsExtension(highlightJsEngine));
            return pipeline;
        }
    }
}
