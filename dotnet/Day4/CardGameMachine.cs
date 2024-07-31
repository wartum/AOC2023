namespace Day4;
public class CardGameMachine
{
    private IEnumerable<Card> _referenceDeck;
    private Dictionary<int, int> _nCache = new();
    private int _maxCardId;
    public CardGameMachine(IEnumerable<Card> referenceDeck)
    {
        _referenceDeck = referenceDeck;
        _maxCardId = _referenceDeck.Last().Id;
        foreach (var card in _referenceDeck)
        {
            _nCache[card.Id] = card.N;
        }
    }

    public int Play()
    {
        return Play(_referenceDeck.Select(c => c.Id));
    }

    private int Play(IEnumerable<int> currentHand)
    {
        var newHand = currentHand.SelectMany(id => ScratchTheCard(id));
        Console.WriteLine(newHand.Count());
        if (newHand == null || newHand.Count() == 0)
        {
            return currentHand.Count();
        }
        else
        {
            return currentHand.Count() + Play(newHand);
        }
    }

    private IEnumerable<int> ScratchTheCard(int cardId)
    {
        for (int i = cardId + 1; i <= _nCache[cardId] + cardId; i++)
        {
            if (i <= +_maxCardId)
            {
                yield return i;
            }
        }
    }
}
