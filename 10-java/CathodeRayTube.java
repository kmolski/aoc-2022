import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Set;
import java.util.stream.Stream;

public class CathodeRayTube {
    public static class HandheldState {
        private final StringBuilder crtDisplay = new StringBuilder();
        private int signalStrengthSum = 0;
    }

    private static List<String> readProgram(String filename) throws IOException {
        try (Stream<String> lines = Files.lines(Paths.get(filename))) {
            return lines.toList();
        }
    }

    private static final Set<Integer> CYCLES_TO_MEASURE = Set.of(20, 60, 100, 140, 180, 220);

    private static void updateSignalStrengthSum(HandheldState handheldState, int currentCycle, int accumulator) {
        if (CYCLES_TO_MEASURE.contains(currentCycle)) {
            handheldState.signalStrengthSum += currentCycle * accumulator;
        }
    }

    private static void updateCrtDisplay(HandheldState handheldState, int currentCycle, int accumulator) {
        handheldState.crtDisplay.append((Math.abs((currentCycle - 1) % 40 - accumulator) <= 1) ? '#' : '.');
        if (currentCycle % 40 == 0) {
            handheldState.crtDisplay.append(System.lineSeparator());
        }
    }

    private static void solve(List<String> program, HandheldState handheldState) {
        var currentCycle = 1;
        var accumulator = 1;
        for (String instruction : program) {
            var tokens = instruction.split(" ", 2);

            var cycles = switch (tokens[0]) {
                case "noop" -> 1;
                case "addx" -> 2;
                default -> throw new IllegalStateException("Unrecognized instruction: " + instruction);
            };

            for (int i = 0; i < cycles; ++i) {
                updateSignalStrengthSum(handheldState, currentCycle, accumulator);
                updateCrtDisplay(handheldState, currentCycle, accumulator);

                ++currentCycle;
            }

            if (tokens[0].equals("addx")) {
                accumulator += Integer.parseInt(tokens[1]);
            }
        }
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            throw new IllegalArgumentException("Input file name not found!");
        }

        try {
            var filename = args[0];
            var program = readProgram(filename);
            var state = new HandheldState();

            solve(program, state);

            System.out.printf("Part 1: %d%n", state.signalStrengthSum);
            System.out.printf("Part 2: %n%s%n", state.crtDisplay);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
