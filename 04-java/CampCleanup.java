import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Stream;

public class CampCleanup {
    public record Range(int start, int end) {
        public boolean fullyContains(Range other) {
            return this.start <= other.start && other.end <= this.end;
        }

        public boolean overlapsWith(Range other) {
            return this.start <= other.end && this.end >= other.start;
        }
    }

    public record Pair<T>(T first, T second) {}

    private static List<Pair<Range>> readPairs(String filename) throws IOException {
        try (Stream<String> lines = Files.lines(Paths.get(filename))) {
            return lines.map(line -> {
                var ranges = line.split(",", 2);
                var firstRange = Arrays.stream(ranges[0].split("-", 2)).mapToInt(Integer::parseInt).toArray();
                var secondRange = Arrays.stream(ranges[1].split("-", 2)).mapToInt(Integer::parseInt).toArray();
                return new Pair<>(new Range(firstRange[0], firstRange[1]),
                                  new Range(secondRange[0], secondRange[1]));
            }).toList();
        }
    }

    private static long part1(List<Pair<Range>> pairs) {
        return pairs.stream()
                .filter(pair -> pair.first().fullyContains(pair.second()) || pair.second().fullyContains(pair.first()))
                .count();
    }

    private static long part2(List<Pair<Range>> pairs) {
        return pairs.stream()
                .filter(pair -> pair.first().overlapsWith(pair.second()))
                .count();
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            throw new IllegalArgumentException("Input file name not found!");
        }

        try {
            var filename = args[0];
            var pairs = readPairs(filename);

            System.out.printf("Part 1: %d%n", part1(pairs));
            System.out.printf("Part 2: %d%n", part2(pairs));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
