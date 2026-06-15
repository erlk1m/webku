import type { Content, ListItem, PhrasingContent, Root, Table, TableCell, TableRow } from 'mdast';
import type { Plugin } from 'unified';

export const remarkMarkdocTable: Plugin<[], Root> = () => {
  return (tree: Root) => {
    const { children } = tree;
    let i = 0;

    while (i < children.length) {
      if (!isTableTag(children[i], '{% table %}')) {
        i++;
        continue;
      }

      const startIdx = i;
      i++;

      const collected: Content[] = [];
      let foundEnd = false;

      while (i < children.length) {
        if (isTableTag(children[i], '{% /table %}')) {
          foundEnd = true;
          i++;
          break;
        }
        collected.push(children[i]);
        i++;
      }

      if (!foundEnd) continue;

      const rows = collectRows(collected);
      if (rows.length === 0) continue;

      const maxCols = Math.max(...rows.map((r) => r.length));

      const table: Table = {
        type: 'table',
        align: Array(maxCols).fill(null),
        children: rows.map(
          (cells): TableRow => ({
            type: 'tableRow',
            children: Array.from(
              { length: maxCols },
              (_, ci): TableCell => ({
                type: 'tableCell',
                children: (cells[ci] || []) as PhrasingContent[],
              }),
            ),
          }),
        ),
      };

      const removeCount = i - startIdx;
      children.splice(startIdx, removeCount, table);
      i = startIdx + 1;
    }
  };
};

function isTableTag(node: Content, tag: string): boolean {
  return (
    node.type === 'paragraph' &&
    node.children.length === 1 &&
    node.children[0].type === 'text' &&
    node.children[0].value.trim() === tag
  );
}

function collectRows(nodes: Content[]): PhrasingContent[][][] {
  const rows: PhrasingContent[][][] = [];
  let currentRow: PhrasingContent[][] = [];

  for (const node of nodes) {
    if (node.type === 'thematicBreak') {
      if (currentRow.length > 0) {
        rows.push(currentRow);
        currentRow = [];
      }
    } else if (node.type === 'list' && node.ordered === false) {
      for (const item of node.children) {
        currentRow.push(getListItemContent(item));
      }
    }
  }

  if (currentRow.length > 0) {
    rows.push(currentRow);
  }

  return rows;
}

function getListItemContent(item: ListItem): PhrasingContent[] {
  if (!item.children?.length) return [];
  const first = item.children[0];
  if (first.type === 'paragraph') {
    return first.children as PhrasingContent[];
  }
  return [];
}
