using System;
using System.Collections.Generic;
using System.Linq;
using System.Timers;

namespace Berrysoft.Pages.KanaLearn
{
    public class Controller
    {
        private readonly static Dictionary<string, string[]> KanaMap = new Dictionary<string, string[]>
        {
            ["あ"] = new string[] { "a" },
            ["ア"] = new string[] { "a" },
            ["い"] = new string[] { "i" },
            ["イ"] = new string[] { "i" },
            ["う"] = new string[] { "u" },
            ["ウ"] = new string[] { "u" },
            ["え"] = new string[] { "e" },
            ["エ"] = new string[] { "e" },
            ["お"] = new string[] { "o" },
            ["オ"] = new string[] { "o" },
            ["か"] = new string[] { "ka" },
            ["カ"] = new string[] { "ka" },
            ["き"] = new string[] { "ki" },
            ["キ"] = new string[] { "ki" },
            ["く"] = new string[] { "ku" },
            ["ク"] = new string[] { "ku" },
            ["け"] = new string[] { "ke" },
            ["ケ"] = new string[] { "ke" },
            ["こ"] = new string[] { "ko" },
            ["コ"] = new string[] { "ko" },
            ["さ"] = new string[] { "sa" },
            ["サ"] = new string[] { "sa" },
            ["し"] = new string[] { "si", "shi" },
            ["シ"] = new string[] { "si", "shi" },
            ["す"] = new string[] { "su" },
            ["ス"] = new string[] { "su" },
            ["せ"] = new string[] { "se" },
            ["セ"] = new string[] { "se" },
            ["そ"] = new string[] { "so" },
            ["ソ"] = new string[] { "so" },
            ["た"] = new string[] { "ta" },
            ["タ"] = new string[] { "ta" },
            ["ち"] = new string[] { "ti", "chi" },
            ["チ"] = new string[] { "ti", "chi" },
            ["つ"] = new string[] { "tu", "tsu" },
            ["ツ"] = new string[] { "tu", "tsu" },
            ["て"] = new string[] { "te" },
            ["テ"] = new string[] { "te" },
            ["と"] = new string[] { "to" },
            ["ト"] = new string[] { "to" },
            ["な"] = new string[] { "na" },
            ["ナ"] = new string[] { "na" },
            ["に"] = new string[] { "ni" },
            ["ニ"] = new string[] { "ni" },
            ["ぬ"] = new string[] { "nu" },
            ["ヌ"] = new string[] { "nu" },
            ["ね"] = new string[] { "ne" },
            ["ネ"] = new string[] { "ne" },
            ["の"] = new string[] { "no" },
            ["ノ"] = new string[] { "no" },
            ["は"] = new string[] { "ha" },
            ["ハ"] = new string[] { "ha" },
            ["ひ"] = new string[] { "hi" },
            ["ヒ"] = new string[] { "hi" },
            ["ふ"] = new string[] { "hu", "fu" },
            ["フ"] = new string[] { "hu", "fu" },
            ["へ"] = new string[] { "he" },
            ["ヘ"] = new string[] { "he" },
            ["ほ"] = new string[] { "ho" },
            ["ホ"] = new string[] { "ho" },
            ["ま"] = new string[] { "ma" },
            ["マ"] = new string[] { "ma" },
            ["み"] = new string[] { "mi" },
            ["ミ"] = new string[] { "mi" },
            ["む"] = new string[] { "mu" },
            ["ム"] = new string[] { "mu" },
            ["め"] = new string[] { "me" },
            ["メ"] = new string[] { "me" },
            ["も"] = new string[] { "mo" },
            ["モ"] = new string[] { "mo" },
            ["や"] = new string[] { "ya" },
            ["ヤ"] = new string[] { "ya" },
            ["ゆ"] = new string[] { "yu" },
            ["ユ"] = new string[] { "yu" },
            ["よ"] = new string[] { "yo" },
            ["ヨ"] = new string[] { "yo" },
            ["ら"] = new string[] { "ra" },
            ["ラ"] = new string[] { "ra" },
            ["り"] = new string[] { "ri" },
            ["リ"] = new string[] { "ri" },
            ["る"] = new string[] { "ru" },
            ["ル"] = new string[] { "ru" },
            ["れ"] = new string[] { "re" },
            ["レ"] = new string[] { "re" },
            ["ろ"] = new string[] { "ro" },
            ["ロ"] = new string[] { "ro" },
            ["わ"] = new string[] { "wa" },
            ["ワ"] = new string[] { "wa" },
            ["を"] = new string[] { "wo", "o" },
            ["ヲ"] = new string[] { "wo", "o" },
            ["ん"] = new string[] { "n", "nn" },
            ["ン"] = new string[] { "n", "nn" },
            ["が"] = new string[] { "ga" },
            ["ガ"] = new string[] { "ga" },
            ["ぎ"] = new string[] { "gi" },
            ["ギ"] = new string[] { "gi" },
            ["ぐ"] = new string[] { "gu" },
            ["グ"] = new string[] { "gu" },
            ["げ"] = new string[] { "ge" },
            ["ゲ"] = new string[] { "ge" },
            ["ご"] = new string[] { "go" },
            ["ゴ"] = new string[] { "go" },
            ["ざ"] = new string[] { "za" },
            ["ザ"] = new string[] { "za" },
            ["じ"] = new string[] { "zi", "ji" },
            ["ジ"] = new string[] { "zi", "ji" },
            ["ず"] = new string[] { "zu" },
            ["ズ"] = new string[] { "zu" },
            ["ぜ"] = new string[] { "ze" },
            ["ゼ"] = new string[] { "ze" },
            ["ぞ"] = new string[] { "zo" },
            ["ゾ"] = new string[] { "zo" },
            ["だ"] = new string[] { "da" },
            ["ダ"] = new string[] { "da" },
            ["ぢ"] = new string[] { "di" },
            ["ヂ"] = new string[] { "di" },
            ["づ"] = new string[] { "du" },
            ["ヅ"] = new string[] { "du" },
            ["で"] = new string[] { "de" },
            ["デ"] = new string[] { "de" },
            ["ど"] = new string[] { "do" },
            ["ド"] = new string[] { "do" },
            ["ば"] = new string[] { "ba" },
            ["バ"] = new string[] { "ba" },
            ["び"] = new string[] { "bi" },
            ["ビ"] = new string[] { "bi" },
            ["ぶ"] = new string[] { "bu" },
            ["ブ"] = new string[] { "bu" },
            ["べ"] = new string[] { "be" },
            ["ベ"] = new string[] { "be" },
            ["ぼ"] = new string[] { "bo" },
            ["ボ"] = new string[] { "bo" },
            ["ぱ"] = new string[] { "pa" },
            ["パ"] = new string[] { "pa" },
            ["ぴ"] = new string[] { "pi" },
            ["ピ"] = new string[] { "pi" },
            ["ぷ"] = new string[] { "pu" },
            ["プ"] = new string[] { "pu" },
            ["ぺ"] = new string[] { "pe" },
            ["ペ"] = new string[] { "pe" },
            ["ぽ"] = new string[] { "po" },
            ["ポ"] = new string[] { "po" },
        };

        private Timer mainTimer = new Timer(3000);

        public Controller()
        {
            mainTimer.AutoReset = true;
            mainTimer.Elapsed += (sender, e) => MainTimer_Tick();
        }

        private bool running;
        public bool Running
        {
            get => running;
            set
            {
                if (running != value)
                {
                    running = value;
                    RunningChanged?.Invoke(this, value);
                }
            }
        }

        public event EventHandler<bool>? RunningChanged;

        public void Start()
        {
            if (kanas.Count == 0) InitializeKanas();
            if (CurrentKana == kanas.Last()) CurrentKana = null;
            if (CurrentKana == null) MainTimer_Tick();
            mainTimer.Start();
            Running = true;
        }

        public void Pause()
        {
            mainTimer.Stop();
            Running = false;
        }

        public void Confirm()
        {
            if (Running)
            {
                Pause();
                if (MainTimer_Tick()) Start();
            }
            else
            {
                Start();
            }
        }

        public string? CurrentKana { get; set; }

        public string? Input { get; set; }

        public string? CorrectAnswer { get; set; }

        private readonly List<string> kanas = new List<string>();

        private void InitializeKanas()
        {
            kanas.Clear();
            kanas.AddRange(KanaMap.Keys.Random());
        }

        public event EventHandler? Ticked;

        private bool MainTimer_Tick()
        {
            CorrectAnswer = null;
            if (CurrentKana != null) kanas.Remove(CurrentKana);
            bool res = CurrentKana == null || KanaMap[CurrentKana].Contains(Input ?? string.Empty);
            if (res)
            {
                CurrentKana = kanas.FirstOrDefault();
            }
            else
            {
                CorrectAnswer = string.Join('/', KanaMap[CurrentKana!]);
                kanas.Add(CurrentKana!);
                Pause();
            }
            Input = null;
            if (CurrentKana == null) Pause();
            Ticked?.Invoke(this, EventArgs.Empty);
            return res;
        }
    }
}
