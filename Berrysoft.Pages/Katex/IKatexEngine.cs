namespace Berrysoft.Pages.Katex
{
    public interface IKatexEngine
    {
        string Run(string code, bool display);
    }
}
