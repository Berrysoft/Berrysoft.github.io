using Microsoft.JSInterop;

namespace Berrysoft.Pages.HighlightJs
{
    public class HighlightJsEngine : IHighlightJsEngine
    {
        protected IJSRuntime JSRuntime { get; set; }

        public HighlightJsEngine(IJSRuntime jSRuntime) => JSRuntime = jSRuntime;

        public string Run(string language, string code)
        {
            return ((IJSInProcessRuntime)JSRuntime).Invoke<string>("highlight", language, code);
        }
    }
}
