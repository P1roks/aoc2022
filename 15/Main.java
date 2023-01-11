import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;
import java.util.stream.Collectors;

class Vec2{
    public int x,y;
    public Vec2(int x,int y){
        this.x = x;
        this.y = y;
    }
    public int getRange(Vec2 other){
        return Math.abs(this.x - other.x) + Math.abs(this.y - other.y);
    }
    @Override
    public boolean equals(Object obj) {
        if(obj instanceof Vec2){
            var other = (Vec2)obj;
            return this.x == other.x && this.y == other.y;
        }
        else
            return false;
    }

    @Override
    public int hashCode() {
        return (this.x * 13) * (this.y * 67);
    }

    @Override
    public String toString() {
        return String.format("%s %s%n",this.x,this.y);
    }
}

public class Main {
    static Set<Vec2> solvePart1(int dist,Vec2 initial,int y){
        var points = new HashSet<Vec2>();

        int farLeft = initial.x - dist;
        int farRight = initial.x + dist;
        int currYUp = initial.y;
        int currYDown = initial.y;

        do{
	    if(currYUp == y || currYDown == y)
		    for(int x = farLeft; x <= farRight; x++){
		       if(currYUp == y)
			       points.add(new Vec2(x,currYUp));
		       else if(currYDown == y)
			       points.add(new Vec2(x,currYDown));
		    }
            currYUp++;
            currYDown--;
            farRight--;
            farLeft++;
        }while(farRight != farLeft);

        points.add(new Vec2(initial.x,initial.y + dist));
        points.add(new Vec2(initial.x,initial.y - dist));

        return points;
    }

    public static void main(String[] args) {
       var points = new HashSet<Vec2>();
       var beacons = new HashSet<Vec2>();
       try{
            var file = new Scanner(new File("./input.txt"));
            while(file.hasNextLine()){
                String line = file.nextLine();
                List<Integer> coords = Arrays.stream(line.split(" "))
                        .skip(2)
                        .filter(x -> x.charAt(0) == 'x' || x.charAt(0) == 'y')
                        .map(x-> Integer.parseInt(x.replaceAll("[^\\d-]","")))
                        .collect(Collectors.toList());
                var sensor = new Vec2(coords.get(0),coords.get(1));
                var beacon = new Vec2(coords.get(2),coords.get(3));
                beacons.add(beacon);

                int range = sensor.getRange(beacon);

                points.addAll(solvePart1(range,sensor,2000000));
            }
       }
       catch(FileNotFoundException e){
           System.out.println("error");
           e.printStackTrace();
       }

        System.out.println(points.stream().count());
    }
}
