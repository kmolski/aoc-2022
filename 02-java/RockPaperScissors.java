import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Stream;

public class RockPaperScissors {
    public interface GameMove {
        int playAgainst(GameMove other);
        int getRoundBonus(GameMove other);
    }

    public enum LiteralMove implements GameMove {
        ROCK {
            public int playAgainst(GameMove other) {
                return (other == SCISSORS) ? 1 : super.playAgainst(other);
            }
        },
        PAPER {
            public int playAgainst(GameMove other) {
                return (other == ROCK) ? 1 : super.playAgainst(other);
            }
        },
        SCISSORS {
            public int playAgainst(GameMove other) {
                return (other == PAPER) ? 1 : super.playAgainst(other);
            }
        };

        public static LiteralMove fromOpponentString(char move) {
            return LiteralMove.values()[move - 'A'];
        }

        public static LiteralMove fromPlayerString(char move) {
            return LiteralMove.values()[move - 'X'];
        }

        public int playAgainst(GameMove other) {
            return (this == other) ? 0 : -1;
        }

        public int getRoundBonus(GameMove other) {
            return this.ordinal() + 1;
        }
    }

    public enum DesiredOutcomeMove implements GameMove {
        LOSS, DRAW, WIN;

        public static DesiredOutcomeMove fromPlayerMove(LiteralMove move) {
            return DesiredOutcomeMove.values()[move.ordinal()];
        }

        private LiteralMove getLiteralMove(GameMove other) {
            return Arrays.stream(LiteralMove.values())
                    .filter(move -> move.playAgainst(other) == this.ordinal() - 1)
                    .findFirst().orElseThrow(() -> new IllegalStateException("Cannot find matching move"));
        }

        public int playAgainst(GameMove other) {
            return getLiteralMove(other).playAgainst(other);
        }

        public int getRoundBonus(GameMove other) {
            return getLiteralMove(other).getRoundBonus(other);
        }
    }

    public record GameScore(int opponentTotal, int playerTotal) {}

    public record GameRound<T extends GameMove>(GameMove opponentMove, T playerMove) {
        public GameScore play(GameScore previousScore) {
            int playerOutcome = playerMove.playAgainst(opponentMove);
            int playerOutcomeBonus = (playerOutcome + 1) * 3;
            int opponentOutcomeBonus = (1 - playerOutcome) * 3;

            int opponentScore = opponentMove.getRoundBonus(playerMove) + opponentOutcomeBonus;
            int playerScore = playerMove.getRoundBonus(opponentMove) + playerOutcomeBonus;
            return new GameScore(previousScore.opponentTotal + opponentScore,
                                 previousScore.playerTotal + playerScore);
        }
    }

    private static List<GameRound<LiteralMove>> readInstructions(String filename) throws IOException {
        try (Stream<String> lines = Files.lines(Paths.get(filename))) {
            return lines.map(line -> new GameRound<>(LiteralMove.fromOpponentString(line.charAt(0)),
                                                     LiteralMove.fromPlayerString(line.charAt(2)))).toList();
        }
    }

    private static <T extends GameMove> int part1(List<GameRound<T>> gameRounds) {
        var score = new GameScore(0, 0);
        for (var round : gameRounds) {
            score = round.play(score);
        }
        return score.playerTotal;
    }

    private static int part2(List<GameRound<LiteralMove>> gameRounds) {
        var convertedInstructions = gameRounds.stream()
                .map(instruction -> {
                    var playerMove = DesiredOutcomeMove.fromPlayerMove(instruction.playerMove);
                    return new GameRound<>(instruction.opponentMove, playerMove);
                }).toList();
        return part1(convertedInstructions);
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            throw new IllegalArgumentException("Input file name not found!");
        }

        try {
            var filename = args[0];
            var gameRounds = readInstructions(filename);

            System.out.printf("Part 1: %d%n", part1(gameRounds));
            System.out.printf("Part 2: %d%n", part2(gameRounds));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
