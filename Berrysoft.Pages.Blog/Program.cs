using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Encodings.Web;
using System.Text.Json;
using System.Text.Unicode;
using Berrysoft.Pages.Data;
using CommandLine;

namespace Berrysoft.Pages.Blog
{
    class Options
    {
        [Option('t', "title", Required = true)]
        public string? Title { get; set; }
        [Option('i', "input", Required = true)]
        public string? InputPath { get; set; }
        [Option('o', "output", Required = true)]
        public string? OutputPath { get; set; }
    }

    class Program
    {
        static void Main(string[] args)
        {
            Parser.Default.ParseArguments<Options>(args).WithParsed(OnParsed);
        }

        static void OnParsed(Options options)
        {
            var file = new FileInfo(options.InputPath!);
            var fn = GetFilename(file);
            var post = new BlogPost
            {
                Title = options.Title!,
                Date = DateTime.Now,
                Filename = fn
            };
            var directory = new DirectoryInfo(options.OutputPath!);
            if (directory.Exists)
            {
                file.MoveTo(Path.Combine(directory.FullName, fn));
                var indexFile = Path.Combine(directory.FullName, "index.json");
                var list = JsonSerializer.Deserialize<List<BlogPost>>(File.ReadAllBytes(indexFile), new JsonSerializerOptions()
                {
                    PropertyNamingPolicy = JsonNamingPolicy.CamelCase
                });
                list.Add(post);
                File.WriteAllBytes(indexFile, JsonSerializer.SerializeToUtf8Bytes(list, new JsonSerializerOptions()
                {
                    WriteIndented = true,
                    PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
                    Encoder = JavaScriptEncoder.Create(new TextEncoderSettings(UnicodeRange.Create('\0', '\x7F'), UnicodeRange.Create('\x4E00', '\x9FBF')))
                }));
            }
            else
            {
                string json = JsonSerializer.Serialize(post, new JsonSerializerOptions()
                {
                    WriteIndented = true,
                    PropertyNamingPolicy = JsonNamingPolicy.CamelCase
                });
                Console.WriteLine(json);
            }
        }

        static string GetFilename(FileInfo file)
        {
            string[] slices = file.Name.Split('.');
            string name = string.Join('.', slices[..^1]);
            var now = DateTime.Now;
            return $"{name}_{now.Year}_{now.Month}_{now.Day}";
        }
    }
}
