using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Net.Http;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;

namespace Berrysoft.Pages.Data
{
    // https://stackoverflow.com/questions/7612602/why-cant-i-use-the-await-operator-within-the-body-of-a-lock-statement
    class SemaphoreLocker
    {
        private readonly SemaphoreSlim _semaphore = new SemaphoreSlim(1, 1);

        public async Task LockAsync(Func<Task> worker)
        {
            await _semaphore.WaitAsync();
            try
            {
                await worker();
            }
            finally
            {
                _semaphore.Release();
            }
        }

        public async Task<T> LockAsync<T>(Func<Task<T>> worker)
        {
            await _semaphore.WaitAsync();
            try
            {
                return await worker();
            }
            finally
            {
                _semaphore.Release();
            }
        }
    }

    public class LocalizationService : ILocalizationService
    {
        private string[] languages;
        private static readonly SemaphoreLocker languagesLocker = new SemaphoreLocker();
        private Dictionary<string, JsonDocument> strings;
        private static readonly SemaphoreLocker stringsLocker = new SemaphoreLocker();

        private string language;
        public string Language
        {
            get => language;
            set
            {
                if (language != value)
                {
                    language = value;
                    LanguageChanged?.Invoke(this, Language);
                }
            }
        }

        public event EventHandler<string> LanguageChanged;

        protected HttpClient Http { get; set; }

        public LocalizationService(HttpClient http)
        {
            Http = http;
            strings = new Dictionary<string, JsonDocument>();
        }

        private async Task<(string, string)> GetStringsFileNameAsync(string lang)
        {
            if (languages == null)
            {
                await languagesLocker.LockAsync(async () =>
                {
                    if (languages == null)
                    {
                        languages = await Http.GetJsonAsync<string[]>("i18n/index.json");
                    }
                });
            }
            while (lang != null && !languages.Contains(lang))
            {
                try
                {
                    var culture = CultureInfo.GetCultureInfo(lang);
                    lang = culture.Parent.Name.ToLower();
                }
                catch (CultureNotFoundException)
                {
                    lang = null;
                }
            }
            if (string.IsNullOrEmpty(lang))
            {
                return (string.Empty, "i18n/strings.json");
            }
            else
            {
                return (lang, $"i18n/strings.{lang}.json");
            }
        }

        private Task<JsonDocument> GetStringsAsync(string lang)
        {
            return stringsLocker.LockAsync(async () =>
            {
                if (strings.TryGetValue(lang, out JsonDocument document))
                {
                    return document;
                }
                else
                {
                    var (realLang, filename) = await GetStringsFileNameAsync(lang);
                    document = JsonDocument.Parse(await Http.GetByteArrayAsync(filename));
                    strings[lang] = document;
                    strings[realLang] = document;
                    return document;
                }
            });
        }

        public async Task<string> GetStringAsync(string key)
        {
            JsonDocument document = await GetStringsAsync(Language ?? string.Empty);
            if (document != null && document.RootElement.TryGetProperty(key, out var prop))
            {
                return prop.GetString();
            }
            else
            {
                return null;
            }
        }
    }
}
