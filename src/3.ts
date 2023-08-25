enum Color {
  Red,
  Green,
  Blue,
}

const printColor = (color: Color) => {
  switch (color) {
    case Color.Red:
      console.log("Red");
      break;
    case Color.Green:
      console.log("Green");
      break;

    case Color.Blue:
      console.log("Blue");
      break;
  }
};

printColor(Color.Red);
