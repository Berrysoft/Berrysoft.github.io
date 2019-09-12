using Markdig;
using Markdig.Extensions.Mathematics;
using Markdig.Renderers;

namespace Berrysoft.Pages.Katex
{
    public class KatexExtension : IMarkdownExtension
    {
        private readonly IKatexEngine katexEngine;
        public KatexExtension(IKatexEngine katexEngine) => this.katexEngine = katexEngine;

        public void Setup(MarkdownPipelineBuilder pipeline) { }

        public void Setup(MarkdownPipeline pipeline, IMarkdownRenderer renderer)
        {
            if (!(renderer is TextRendererBase<HtmlRenderer> htmlRenderer))
            {
                return;
            }

            var originalBlockRenderer = htmlRenderer.ObjectRenderers.FindExact<HtmlMathBlockRenderer>();
            if (originalBlockRenderer != null)
            {
                htmlRenderer.ObjectRenderers.Remove(originalBlockRenderer);
            }
            htmlRenderer.ObjectRenderers.Insert(0, new KatexHtmlMathBlockRenderer(katexEngine));

            var originalInlineRenderer = htmlRenderer.ObjectRenderers.FindExact<HtmlMathInlineRenderer>();
            if (originalInlineRenderer != null)
            {
                htmlRenderer.ObjectRenderers.Remove(originalInlineRenderer);
            }
            htmlRenderer.ObjectRenderers.Insert(0, new KatexHtmlMathInlineRenderer(katexEngine));
        }
    }
}
