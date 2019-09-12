using System;
using Markdig.Extensions.Mathematics;
using Markdig.Renderers;
using Markdig.Renderers.Html;

namespace Berrysoft.Pages.Katex
{
    public class KatexHtmlMathInlineRenderer : HtmlObjectRenderer<MathInline>
    {
        private readonly IKatexEngine katexEngine;

        public KatexHtmlMathInlineRenderer(IKatexEngine katexEngine) => this.katexEngine = katexEngine;

        protected override void Write(HtmlRenderer renderer, MathInline obj)
        {
            renderer.WriteLine(katexEngine.Run(obj.Content.ToString(), false));
        }
    }
}
