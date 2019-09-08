using Microsoft.JSInterop;

namespace Berrysoft.Pages.HighlightJs
{
    public class HighlightJsEngine : IHighlightJsEngine
    {
        protected IJSInProcessRuntime JSRuntime { get; set; }

        public HighlightJsEngine(IJSRuntime jSRuntime) => JSRuntime = (IJSInProcessRuntime)jSRuntime;

        public string Run(string language, string code)
        {
            return JSRuntime.Invoke<string>("highlight", language, code);
        }
    }
}
