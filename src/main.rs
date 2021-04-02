use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;

type Perm = Vec<usize>;

fn compose(a : &Perm, b: &Perm) -> Perm {
    let mut c = Vec::new();
    for &i in a {
        c.push(b[i]);
    }
    return c;
}

fn inverse(a : &Perm) -> Perm {
    let n = a.len();
    let mut c = vec![0; n];
    for i in 0..n {
        let j = a[i];
        c[j] = i;
    }
    return c;
}

fn identity(n : usize) -> Perm {
    return (0..n).collect();
}

fn conj(a : &Perm, b : &Perm) -> Perm {
    return compose(&compose(&inverse(a),b),a);
}

fn random(n : usize) -> Perm {
    let mut a = identity(n);
    a.shuffle(&mut thread_rng());
    return a;
}

fn swap(n : usize, x : usize, y : usize) -> Perm {
    let mut a = identity(n);
    a[x] = y;
    a[y] = x;
    return a;
}

fn randomSwap(n : usize) -> Perm {
    return swap(n, 
        thread_rng().gen_range(0,n),
        thread_rng().gen_range(0,n)
    );
}

fn halfRandomSwap(n : usize, support : &Vec<usize>) -> Perm {
    if support.len() == 0 {
        return randomSwap(n);
    }
    return swap(n, 
        *support.choose(&mut thread_rng()).unwrap(),
        thread_rng().gen_range(0,n),
    );
}

fn alter(a : &Perm) -> Perm {
    return compose(&a, &randomSwap(a.len()));
}

fn alterK(a : &Perm, k : u32) -> Perm {
    let mut b = a.clone();
    for _ in 0..k {
        b = alter(&b);
    }
    return b;
}

fn power(a :&Perm, k : u32) -> Perm {
    let n = a.len();
    let mut b = identity(n);
    for _ in 0..k {
        b = compose(&b, &a);
    }
    return b;
}

fn weight(a : &Perm) -> i32 {
    let n = a.len();
    let mut k = 0;
    for i in 0..n {
        if a[i] == i {
            k +=1;
        }
    }
    return (n as i32) - k;
}

fn support(a : &Perm) -> Vec<usize> {
    let mut support = Vec::new();
    for i in 0..a.len() {
        if i != a[i] {
            support.push(i);
        }
    }
    return support;
}

fn makeDelta(a : &Perm) -> Perm {
    if thread_rng().gen_range(0,10) < 1 {
        return inverse(&a);
    }
    let support = support(a);
    if support.len() > 0 {
        let mut delta = identity(a.len());
        for _ in 0..3 {
            let i = *support.choose(&mut thread_rng()).unwrap();
            delta = compose(&delta, &swap(a.len(), i, a[i]))
        }
        return delta;
    } else {
        return identity(a.len());
    }
}

fn dist(a : &Perm, b : &Perm) -> i32 {
    return weight(&compose(&inverse(&a), &b));
}

fn showCycles(a : &Perm) {
    let mut a = a.clone();
    let n = a.len();
    let mut cycles = Vec::new();
    'outer : loop {
        for i in 0..n {
            if a[i] != n { //bad i know
                let mut cycle = Vec::new();
                let mut j = i;
                while a[j] != n {
                    cycle.push(j);
                    let j2 = a[j];
                    a[j] = n;
                    j = j2;
                }

                let &min = cycle.iter().min().unwrap();
                while cycle[0] != min {
                    cycle.rotate_left(1);
                }

                cycles.push(cycle);
                continue 'outer;
            }
        }
        break 'outer;
    }
    cycles.sort();
    println!("{:?}", cycles);
    return;
}

fn main() {

    let N = 30;
  
    let a = random(N);
    let a2 = power(&a, 2);

    let mut x = random(N);

    for t in 1..1234 {
        let wx = dist(&power(&x,2),&a2);
        let err = compose(&power(&x,2),&inverse(&a2));
        let support = support(&err);
        if support.len() == 0 {
            continue;
        }

        let i = *support.choose(&mut thread_rng()).unwrap();
        let delta = swap(N, i, err[i]);
        
        // let delta = swap(N, 
        //     *support.choose(&mut thread_rng()).unwrap(), 
        //     *support.choose(&mut thread_rng()).unwrap()
        // );
        // let y = if t % 2 == 0 { compose(&delta, &x) } else { compose(&x, &delta) };
        let y = compose(&delta, &x);
        let wy = dist(&power(&y,2),&a2);
        if wy <= wx {
            if wy < wx {
                println!("progress {} {}", t, wy);
            }
            x = y;
        }
    }

    showCycles(&x);
    println!("");
    showCycles(&a);
    println!("");
    println!("");
    showCycles(&power(&x,2));
    println!("");
    showCycles(&a2);
    println!("");
    println!("");
    showCycles(&compose(&inverse(&power(&x,2)),&a2));
    println!("");
    showCycles(&compose(&inverse(&x),&a));

    return;

    let mut points = vec![random(N), random(N), random(N), random(N), random(N), random(N)];
    // let mut triples = vec![vec![0, 0, 1], vec![1, 1, 2]];
    let mut triples = vec![vec![0, 0, 1], vec![1, 1, 2], vec![2, 2, 3], vec![3, 3, 4], vec![4, 4, 5]];

    fn score(points : &Vec<Perm>, triples : &Vec<Vec<usize>>) -> Vec<i32> { 
        let mut score = 0;
        let mut scores = Vec::new();
        for triple in triples {
            let a = &points[triple[0]];
            let b = &points[triple[1]];
            let mut c = &identity(points[0].len());
            if triple[2] != 666 {
                c = &points[triple[2]];
            }
            scores.push(dist(&compose(a,b),c));
            // score += dist(&compose(a,b),c);
        }
        for point in points { 
            scores.push(-10*weight(point));
            // score -= 10*weight(point);
        }
        return scores;
        // return scores.iter().sum();
        // return score;
    }

    for t in 0..12345 {
        let w : i32 = score(&points, &triples).iter().sum();

        let triple = triples.choose(&mut thread_rng()).unwrap();

        let a = points[triple[0]].clone();
        let b = points[triple[1]].clone();
        let c = points[triple[2]].clone();

        let error = compose(&compose(&a,&b),&inverse(&c));

        match thread_rng().gen_range(0,3) {
            0 => { 
                points[triple[0]] = compose(&makeDelta(&error),&a);
            }
            1 => { 
                points[triple[1]] = compose(&makeDelta(&conj(&a, &error)),&b);
            }
            _ => {
                points[triple[2]] = compose(&makeDelta(&conj(&b, &conj(&a, &error))),&c);
            }
        };

        let nw : i32 = score(&points, &triples).iter().sum();

        if nw <= w {
            if nw < w {
                println!("progress {}\t {:?}", t, score(&points, &triples));
            }
        } else {
            // roll back changes
            points[triple[0]] = a;
            points[triple[1]] = b;
            points[triple[2]] = c;
        }
    }

    println!("{:?}", score(&points, &triples));

    for point in points { 
        showCycles(&point);
    }

    // println!("");

    // showCycles(&x);
    // showCycles(&a);
    // println!("");
    // showCycles(&power(&x,2));
    // showCycles(&sq);
    // println!("");
    // showCycles(&compose(&inverse(&power(&x,2)), &sq));
    // println!("Hello, world!");
}
