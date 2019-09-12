using Markdig.Extensions.Mathematics;
using Markdig.Renderers;
using Markdig.Renderers.Html;

namespace Berrysoft.Pages.Katex
{
    public class KatexHtmlMathBlockRenderer : HtmlObjectRenderer<MathBlock>
    {
        private readonly IKatexEngine katexEngine;

        public KatexHtmlMathBlockRenderer(IKatexEngine katexEngine) => this.katexEngine = katexEngine;

        protected override void Write(HtmlRenderer renderer, MathBlock obj)
        {
            renderer.EnsureLine();
            renderer.WriteLine(katexEngine.Run(string.Join("\r\n", obj.Lines.Lines), true));
        }
    }
}
