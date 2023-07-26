#include <stdio.h>
#include <stdlib.h>

void pointer_example()
{
  int x = 5;
  int *px = malloc(sizeof(int));
  // *assume* malloc succuess then:
  *px = x;
  int *py = px;

  printf("*px: %d\n", *px);
  printf("addr of  *px point to: %p\n", px);
  printf("addr of  *py point to: %p\n", py);

  free(px); // free(py) --> free px or py doing the same thing,maybe check out musl-C-lib-malloc()

  printf("*py after freed: %d\n", *py);
  printf("why this not cause segment-fault? just because py still hold its addr?\n");

  px = NULL;
  py = NULL;

  int *p_random;

  printf("*p_random: like a random electronic-wire flying around the room(mem-addr-space)!!\n");
  printf("why this definately cause segment-fault(means illegal read data area)\n");
  printf("%d", *p_random);
}
int main()
{
  pointer_example();
  return 0;
}