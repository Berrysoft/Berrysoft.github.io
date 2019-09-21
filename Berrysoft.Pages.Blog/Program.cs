using System;
using System.IO;
using System.Linq;
using System.ServiceModel.Syndication;
using System.Xml;
using CommandLine;
using Markdig;

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
            string description;
            using (var reader = new StreamReader(file.FullName))
            {
                description = Markdown.ToPlainText(reader.ReadLine() ?? string.Empty).Trim(' ', '\n', '\r');
            }
            var fn = GetFilename(file);
            var item = new SyndicationItem
            {
                Title = new TextSyndicationContent(options.Title!),
                LastUpdatedTime = DateTime.Now,
                Summary = new TextSyndicationContent(description)
            };
            item.Links.Add(new SyndicationLink(new Uri($"https://berrysoft.github.io/blog/{fn}")));
            var directory = new DirectoryInfo(options.OutputPath!);
            file.MoveTo(Path.Combine(directory.FullName, fn + ".md"));
            var indexFile = Path.Combine(directory.FullName, "rss.xml");
            SyndicationFeed feed;
            using (var stream = new FileStream(indexFile, FileMode.Open))
            using (var reader = XmlReader.Create(stream))
            {
                feed = SyndicationFeed.Load(reader);
            }
            feed.Items = feed.Items.Append(item);
            using (var stream = new FileStream(indexFile, FileMode.OpenOrCreate))
            using (var writer = XmlWriter.Create(stream, new XmlWriterSettings()
            {
                Indent = true
            }))
            {
                feed.SaveAsRss20(writer);
                writer.Flush();
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
