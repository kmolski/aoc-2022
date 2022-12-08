import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class TreetopTreeHouse {
    private static int[][] readForest(String filename) throws IOException {
        try (Stream<String> lines = Files.lines(Paths.get(filename))) {
            return lines.map(line -> line.chars().map(Character::getNumericValue).toArray()).toArray(int[][]::new);
        }
    }

    public record Coord2D(int h, int w) {
        public int absoluteDiff(Coord2D other) {
            return Math.abs(this.h - other.h) + Math.abs(this.w - other.w);
        }
    }

    private static void markVisibleTrees(int[][] forest, int[][] visibility, int[][] scenicScores,
                                         Coord2D edgeCoord, Stream<Coord2D> coords) {
        final int[] max = {Integer.MIN_VALUE};
        final Coord2D[] heightIndices = new Coord2D[10];
        coords.forEachOrdered(coord -> {
            var treeHeight = forest[coord.h][coord.w];
            if (treeHeight > max[0]) {
                visibility[coord.h][coord.w] = 1;
                max[0] = treeHeight;
            }

            int minIndexDiff = Integer.MAX_VALUE;
            for (int i = 9; i >= treeHeight; --i) {
                if (heightIndices[i] != null && heightIndices[i].absoluteDiff(coord) < minIndexDiff) {
                    minIndexDiff = heightIndices[i].absoluteDiff(coord);
                }
            }
            scenicScores[coord.h][coord.w] *= (minIndexDiff == Integer.MAX_VALUE) ? edgeCoord.absoluteDiff(coord) : minIndexDiff;
            heightIndices[treeHeight] = coord;
        });
    }

    private static void walkAllDirections(int[][] forest, int height, int width, int[][] visibility, int[][] scenicScores) {
        IntStream.iterate(0, h -> h < height, h -> h + 1).forEach(h -> // west-to-east
                markVisibleTrees(forest, visibility, scenicScores, new Coord2D(h, 0),
                        IntStream.iterate(0, w -> w < width, w -> w + 1).mapToObj(w -> new Coord2D(h, w))));
        IntStream.iterate(0, h -> h < height, h -> h + 1).forEach(h -> // east-to-west
                markVisibleTrees(forest, visibility, scenicScores, new Coord2D(h, width - 1),
                        IntStream.iterate(width - 1, w -> w >= 0, w -> w - 1).mapToObj(w -> new Coord2D(h, w))));
        IntStream.iterate(0, w -> w < width, w -> w + 1).forEach(w -> // north-to-south
                markVisibleTrees(forest, visibility, scenicScores, new Coord2D(0, w),
                        IntStream.iterate(0, h -> h < height, h -> h + 1).mapToObj(h -> new Coord2D(h, w))));
        IntStream.iterate(0, w -> w < width, w -> w + 1).forEach(w -> // south-to-north
                markVisibleTrees(forest, visibility, scenicScores, new Coord2D(height - 1, w),
                        IntStream.iterate(height - 1, h -> h >= 0, h -> h - 1).mapToObj(h -> new Coord2D(h, w))));
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            throw new IllegalArgumentException("Input file name not found!");
        }

        try {
            var filename = args[0];
            var forest = readForest(filename);

            var height = forest.length;
            var width = forest[0].length;
            var visibility = new int[height][width];
            var scenicScores = new int[height][width];
            for (int[] row : scenicScores) { Arrays.fill(row, 1); }

            walkAllDirections(forest, height, width, visibility, scenicScores);

            var part1 = Arrays.stream(visibility).mapToLong(row -> Arrays.stream(row).filter(i -> i == 1).count()).sum();
            var part2 = Arrays.stream(scenicScores).mapToLong(row -> Arrays.stream(row).max().getAsInt()).max().getAsLong();

            System.out.printf("Part 1: %d%n", part1);
            System.out.printf("Part 2: %d%n", part2);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
