// Global Scope
// Avoid global scope when possible
int g_NumPersons = 0;

void some_other_function() {
    int personID = 1;
}

int main() {
    // Scope of main function
    // type name = initial value;
    int personID = 0;
    personID += 1;
    {
        // sub-scopt within function's local scope
        // Only the personID of the lowest scope will be affected
        int personID = 0;

        personID += 1;
    }

}
