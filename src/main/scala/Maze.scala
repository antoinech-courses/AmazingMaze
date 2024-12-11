import scala.collection.mutable.ListBuffer

object AmazingMaze extends App:

  enum Exploration:
    case Explored, UnExplored

  enum Maze:
    case Branch(label: String, left: Maze, right: Maze, var status: Exploration = Exploration.UnExplored)
    case Leaf(label: String)

    def explore(trace: ListBuffer[String]): Unit = this match
      case branch@Branch(label, left, right, status) =>
        status match
          case Exploration.UnExplored =>
            branch.status = Exploration.Explored
            trace += label
            left.explore(trace)
            right.explore(trace)
          case Exploration.Explored =>
            trace += label
      case Leaf(label) =>
        trace += label

  val leaf2 = Maze.Leaf("2")
  val leaf4 = Maze.Leaf("4")
  val leaf5 = Maze.Leaf("5")
  val leaf8 = Maze.Leaf("8")
  val branch3 = Maze.Branch("3", leaf4, leaf5)
  val branch1 = Maze.Branch("1", leaf2, branch3)
  val branch7 = Maze.Branch("7", leaf5, leaf8)
  val branch6 = Maze.Branch("6", branch3, branch7)
  val branch0 = Maze.Branch("0", branch1, branch6)

  val trace = ListBuffer[String]()
  branch0.explore(trace)
  println(trace.toList)