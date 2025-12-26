#include <stdio.h>
#include <string.h>

int solve(char *line, int *solution);

int main() {
    char line[100];
    int solution;

    while (1) {
    printf("> ");
    if (fgets(line, 100, stdin) == NULL) {
        return 0;
    }

    if (solve(line, &solution)) {
        continue;
    }

    printf("%d\n", solution);
    
    }
    return 0;
}
