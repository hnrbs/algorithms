<?php

$operations = 0;

function removeItem(array &$items, $index)
{
    unset($items[$index]);
    $items = array_values($items);
}

function findSmallestIndex(array $items, &$operations): int {
    $smallestIndex = 0;

    foreach($items as $key => $item) {
        $operations++;
        if ($item > $items[$smallestIndex]) {
            continue;
        }

        $smallestIndex = $key;
    }

    return $smallestIndex;
}

function selectSort(array $items, &$operations): array {
    $sortedItems = [];

    foreach(range(1, count($items)) as $index)
    {
        $operations++;
        $smallestIndex = findSmallestIndex($items, $operations);

        $sortedItems[] = $items[$smallestIndex];

        removeItem($items, $smallestIndex);
    }

    return $sortedItems;
}

$items = [9, 6, 4, 3, 6, 2];

var_dump(selectSort($items, $operations));
var_dump($operations);

