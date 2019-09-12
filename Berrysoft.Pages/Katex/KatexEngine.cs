using Microsoft.JSInterop;

namespace Berrysoft.Pages.Katex
{
    public class KatexEngine : IKatexEngine
    {
        protected IJSInProcessRuntime JSRuntime { get; set; }

        public KatexEngine(IJSRuntime jSRuntime) => JSRuntime = (IJSInProcessRuntime)jSRuntime;

        public string Run(string code, bool display)
        {
            return JSRuntime.Invoke<string>("katexRender", code, display);
        }
    }
}
