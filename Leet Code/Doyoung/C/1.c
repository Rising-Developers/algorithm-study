#include <stdio.h>
#include <stdlib.h>

// Define a struct for hash table entry
typedef struct {
    int key;
    int value;
} HashTableEntry;

// Define the hash table
typedef struct {
    HashTableEntry *entries;
    int size;
} HashTable;

// Create a hash table
HashTable* createHashTable(int size) {
    HashTable* table = (HashTable*) malloc(sizeof(HashTable));
    table->entries = (HashTableEntry*) calloc(size, sizeof(HashTableEntry));
    table->size = size;
    return table;
}

// Hash function to calculate the index
int hash(int key, int size) {
    return abs(key) % size;
}

// Insert a key-value pair into the hash table
void insert(HashTable *table, int key, int value) {
    int index = hash(key, table->size);
    while (table->entries[index].key != 0) {
        index = (index + 1) % table->size;
    }
    table->entries[index].key = key;
    table->entries[index].value = value;
}

// Search for a key in the hash table and return its value
int search(HashTable *table, int key) {
    int index = hash(key, table->size);
    while (table->entries[index].key != 0) {
        if (table->entries[index].key == key) {
            return table->entries[index].value;
        }
        index = (index + 1) % table->size;
    }
    return -1;  // Indicates that the key is not found
}

// Free the hash table
void freeHashTable(HashTable *table) {
    free(table->entries);
    free(table);
}

// Two Sum function
int* twoSum(int* nums, int numsSize, int target, int* returnSize) {
    HashTable *table = createHashTable(numsSize);
    int *result = (int*) malloc(2 * sizeof(int));
    
    for (int i = 0; i < numsSize; i++) {
        int complement = target - nums[i];
        int complementIndex = search(table, complement);
        if (complementIndex != -1) {
            result[0] = complementIndex;
            result[1] = i;
            *returnSize = 2;
            freeHashTable(table);
            return result;
        }
        insert(table, nums[i], i);
    }
    
    *returnSize = 0;
    freeHashTable(table);
    return NULL;  // No solution found
}
