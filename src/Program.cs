namespace DungeonSaver;

class Program
{
    static void Main(string[] args)
    {
        bool showRoomIds = args.Contains("--room-ids");
        var game = new GameLoop(showRoomIds: showRoomIds);
        game.Run();
    }
}
