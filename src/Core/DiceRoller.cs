namespace DungeonSaver.Core;

/// <summary>
/// Handles all dice rolling for dungeon generation
/// </summary>
public class DiceRoller
{
    private readonly Random _random;

    public DiceRoller(int? seed = null)
    {
        _random = seed.HasValue ? new Random(seed.Value) : new Random();
    }

    /// <summary>
    /// Roll a single D6 (1-6)
    /// </summary>
    public int D6() => _random.Next(1, 7);

    /// <summary>
    /// Roll 2D6 and return both dice separately
    /// </summary>
    /// <returns>(primary die, secondary die)</returns>
    public (int primary, int secondary) Roll2D6()
    {
        return (D6(), D6());
    }

    /// <summary>
    /// Roll D66 - two dice where first is tens, second is ones
    /// Used for room dimensions: primary = X-axis, secondary = Y-axis
    /// </summary>
    public (int x, int y) RollD66()
    {
        return Roll2D6();
    }

    /// <summary>
    /// Check if a 2D6 roll is a double (both dice same value)
    /// </summary>
    public bool IsDouble(int die1, int die2) => die1 == die2;

    /// <summary>
    /// Check if a roll is double-6
    /// </summary>
    public bool IsDouble6(int die1, int die2) => die1 == 6 && die2 == 6;
}
