//Simple snake game on HolyC. build on templeOS or fork. made by DreamModule aka TeaDev aka Idk with love
#define GRID_SIZE 20
#define HEAD_COLOR GREEN 
#define BODY_COLOR LTGREEN
#define FOOD_COLOR RED
#define BG_COLOR BLACK

class SnakePart {
  I64 x, y;
};

SnakePart snake[1024];
I64 snake_len;
I64 dir_x, dir_y;
I64 food_x, food_y;
I64 score;
Bool game_over;

U0 InitGame() {
  snake_len = 5;
  score = 0;
  game_over = FALSE;
  dir_x = 1;
  dir_y = 0;
  I64 i;
  for (i=0; i<snake_len; i++) {
    snake[i].x = 10 - i;
    snake[i].y = 10;
  }
  food_x = 15;
  food_y = 15;
}

U0 SpawnFood() {
  food_x = RandU16 % (GR_WIDTH / GRID_SIZE);
  food_y = RandU16 % (GR_HEIGHT / GRID_SIZE);
}

U0 DrawGame(CDC *dc) {
  dc->color = BG_COLOR;
  GrRect(dc, 0, 0, GR_WIDTH, GR_HEIGHT);
  dc->color = FOOD_COLOR;
  GrCircle(dc, food_x*GRID_SIZE + GRID_SIZE/2, food_y*GRID_SIZE + GRID_SIZE/2, GRID_SIZE/2 - 2);
  GrFloodFill(dc, food_x*GRID_SIZE + GRID_SIZE/2, food_y*GRID_SIZE + GRID_SIZE/2);
  I64 i;
  for (i=0; i<snake_len; i++) {
    if (i == 0) 
      dc->color = HEAD_COLOR;
    else 
      dc->color = BODY_COLOR;
    GrRect(dc, snake[i].x*GRID_SIZE + 1, snake[i].y*GRID_SIZE + 1, GRID_SIZE - 2, GRID_SIZE - 2);
  }
  dc->color = WHITE;
  GrPrint(dc, 10, 10, "Score: %d", score);
  if (game_over) {
    dc->color = RED;
    GrPrint(dc, GR_WIDTH/2 - 40, GR_HEIGHT/2, "GAME OVER");
  }
}

U0 UpdateSnake() {
  if (game_over) return;
  I64 i;
  for (i=snake_len-1; i>0; i--) {
    snake[i].x = snake[i-1].x;
    snake[i].y = snake[i-1].y;
  }
  snake[0].x += dir_x;
  snake[0].y += dir_y;
  if (snake[0].x == food_x && snake[0].y == food_y) {
    snake_len++;
    score += 10;
    SpawnFood();
    if (snake_len >= 1024) snake_len = 1023;
  }
  if (snake[0].x < 0) snake[0].x = (GR_WIDTH / GRID_SIZE) - 1;
  if (snake[0].x >= (GR_WIDTH / GRID_SIZE)) snake[0].x = 0;
  if (snake[0].y < 0) snake[0].y = (GR_HEIGHT / GRID_SIZE) - 1;
  if (snake[0].y >= (GR_HEIGHT / GRID_SIZE)) snake[0].y = 0;
  for (i=1; i<snake_len; i++) {
    if (snake[0].x == snake[i].x && snake[0].y == snake[i].y) {
      game_over = TRUE;
      Beep;
    }
  }
}

U0 SnakeGame() {
  CDC *dc = DCAlias;
  InitGame();
  try {
    while (TRUE) {
      if (ScanKey(SC_CURSOR_UP) && dir_y != 1) { dir_x=0; dir_y=-1; }
      if (ScanKey(SC_CURSOR_DOWN) && dir_y != -1) { dir_x=0; dir_y=1; }
      if (ScanKey(SC_CURSOR_LEFT) && dir_x != 1) { dir_x=-1; dir_y=0; }
      if (ScanKey(SC_CURSOR_RIGHT) && dir_x != -1) { dir_x=1; dir_y=0; }
      if (ScanKey(SC_ESC) && (KB_SC_LSHIFT || KB_SC_RSHIFT)) 
        break;
      UpdateSnake;
      DrawGame(dc);
      Sleep(100);
    }
  } catch {
    DCDel(dc);
  }
  DCDel(dc);
  DCFill;
}

SnakeGame;
