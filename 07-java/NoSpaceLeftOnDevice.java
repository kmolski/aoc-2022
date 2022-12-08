import java.io.IOException;
import java.nio.file.Paths;
import java.util.*;

public class NoSpaceLeftOnDevice {
    public sealed interface FilesystemNode {
        String name();
        int size();
    }

    public record File(String name, int size) implements FilesystemNode {}

    public record Directory(String name, Map<String, FilesystemNode> children) implements FilesystemNode {
        public Directory(String name) {
            this(name, new HashMap<>());
        }

        @Override
        public int size() {
            return this.children.values().stream().mapToInt(FilesystemNode::size).sum();
        }
    }

    public record ShellState(Deque<Directory> directoryStack) {
        public ShellState() {
            this(new ArrayDeque<>());
            this.directoryStack.addLast(new Directory(""));
        }

        @SuppressWarnings("unchecked") // assuming that fs nodes will not be reassigned
        private <T extends FilesystemNode> T createFsNodeInCurrentDir(T node) {
            var currentDir = this.directoryStack.getLast();
            return (T) currentDir.children.putIfAbsent(node.name(), node);
        }

        public Directory createSubdirectory(String name) {
            var directory = new Directory(name);
            return createFsNodeInCurrentDir(directory);
        }

        public File createFileInCurrentDir(String name, int size) {
            var file = new File(name, size);
            return createFsNodeInCurrentDir(file);
        }
    }

    private static final String DIR_OR_SIZE = "(dir|[0-9]+)";

    private static void readDirectoryListing(ShellState shellState, Scanner scanner) {
        while (scanner.hasNext(DIR_OR_SIZE)) {
            var token = scanner.next(DIR_OR_SIZE).strip();
            switch (token) {
                case "dir" -> shellState.createSubdirectory(scanner.next().strip());
                default -> {
                    var size = Integer.parseInt(token);
                    shellState.createFileInCurrentDir(scanner.next().strip(), size);
                }
            }
            scanner.nextLine();
        }
    }

    private static void readCommand(ShellState shellState, Scanner scanner) {
        var line = scanner.nextLine();
        switch (line.strip()) {
            case "$ ls" -> readDirectoryListing(shellState, scanner);
            case "$ cd /" -> {
                var rootDir = shellState.directoryStack.removeFirst();
                shellState.directoryStack.clear();
                shellState.directoryStack.addLast(rootDir);
            }
            case "$ cd .." -> shellState.directoryStack().removeLast();
            default -> { // $ cd <subdirectory>
                var subDirName = line.split(" ", 3)[2];
                var subDir = shellState.createSubdirectory(subDirName);
                shellState.directoryStack.addLast(subDir);
            }
        }
    }

    private static ShellState readShellState(String filename) throws IOException {
        var shellState = new ShellState();
        try (var scanner = new Scanner(Paths.get(filename))) {
            while (scanner.hasNextLine()) {
                readCommand(shellState, scanner);
            }
        }
        return shellState;
    }

    private static Set<Directory> collectDirs(Directory dir, Set<Directory> directories) {
        var finalDirectories = (directories == null) ? new HashSet<Directory>() : directories;

        finalDirectories.add(dir);
        dir.children().forEach((__, child) -> {
            if (child instanceof Directory childDir) {
                collectDirs(childDir, finalDirectories);
            }
        });

        return finalDirectories;
    }

    private static int part1(ShellState shellState) {
        return collectDirs(shellState.directoryStack.getFirst(), null).stream()
                .filter(dir -> dir.size() <= 100_000)
                .mapToInt(Directory::size)
                .sum();
    }

    private static int part2(ShellState shellState) {
        var availableSize = 70_000_000 - shellState.directoryStack.getFirst().size();
        return collectDirs(shellState.directoryStack.getFirst(), null).stream()
                .filter(dir -> availableSize + dir.size() >= 30_000_000)
                .min(Comparator.comparing(Directory::size))
                .get().size();
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            throw new IllegalArgumentException("Input file name not found!");
        }

        try {
            var filename = args[0];
            var shellState = readShellState(filename);

            System.out.printf("Part 1: %d%n", part1(shellState));
            System.out.printf("Part 2: %d%n", part2(shellState));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
