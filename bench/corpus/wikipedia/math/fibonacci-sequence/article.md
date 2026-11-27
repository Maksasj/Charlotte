# Fibonacci sequence

In mathematics, the Fibonacci sequence is a sequence in which each element is the sum of the two elements that precede it.  Numbers that are part of the Fibonacci sequence are known as Fibonacci numbers, commonly denoted Fn . The initial elements of the sequence are F1 = 1 and F2 = 1, though many authors also include a zeroth element F0 = 0. Starting from F0, the sequence begins

0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, ... (sequence A000045 in the OEIS)

The Fibonacci numbers were first described in Indian mathematics as early as 200 BC in work by Pingala on enumerating possible patterns of Sanskrit poetry formed from syllables of two lengths. They are named after the Italian mathematician Leonardo of Pisa, also known as Fibonacci, who introduced the sequence to Western European mathematics in his 1202 book Liber Abaci.

Fibonacci numbers appear unexpectedly often in mathematics, so much so that there is an entire journal dedicated to their study, the Fibonacci Quarterly. Applications of Fibonacci numbers include computer algorithms such as the Fibonacci search technique and the Fibonacci heap data structure, and graphs called Fibonacci cubes used for interconnecting parallel and distributed systems. They also appear in biological settings, such as branching in trees, the arrangement of leaves on a stem, the fruit sprouts of a pineapple, the flowering of an artichoke, and the arrangement of a pine cone's bracts, though they do not occur in all species.

Fibonacci numbers are also strongly related to the golden ratio: Binet's formula expresses the n-th Fibonacci number in terms of n and the golden ratio, and implies that the ratio of two consecutive Fibonacci numbers tends to the golden ratio as n increases. Fibonacci numbers are also closely related to Lucas numbers, which obey the same recurrence relation and with the Fibonacci numbers form a complementary pair of Lucas sequences.

## Definition

The Fibonacci numbers may be defined by the recurrence relation

F0=0,F1=1,

and

Fn=Fn−1+Fn−2

for n > 1.

Under some older definitions, the value F0=0 is omitted, so that the sequence starts with F1=F2=1.

The first 21 Fibonacci numbers Fn are:

The Fibonacci sequence can be extended to negative integer indices by following the same recurrence relation in the negative direction (sequence A039834 in the OEIS): ⁠F1=1⁠, ⁠F0=0⁠, and ⁠Fn=Fn+2−Fn+1⁠ for n < 0 . Nearly all properties of Fibonacci numbers do not depend upon whether the indices are positive or negative. The values for positive and negative indices obey the relation:

F−n=(−1)n+1Fn.

### India

The Fibonacci sequence appears in Indian mathematics, in connection with Sanskrit prosody. In the Sanskrit poetic tradition, there was interest in enumerating all patterns of long (L) syllables of 2 units duration, juxtaposed with short (S) syllables of 1 unit duration. Counting the different patterns of successive L and S with a given total duration results in the Fibonacci numbers: the number of patterns of duration m units is Fm+1.

Knowledge of the Fibonacci sequence was expressed as early as Pingala (c. 450 BC–200 BC). Singh cites Pingala's cryptic formula misrau cha ("the two are mixed") and scholars who interpret it in context as saying that the number of patterns for m beats (Fm+1) is obtained by adding one [S] to the Fm cases and one [L] to the Fm−1 cases. Bharata Muni also expresses knowledge of the sequence in the Natya Shastra (c. 100 BC–c. 350 AD).

However, the clearest exposition of the sequence arises in the work of Virahanka (c. 700 AD), whose own work is lost, but is available in a quotation by Gopala (c. 1135):

Variations of two earlier meters [is the variation] ...  For example, for [a meter of length] four, variations of meters of two [and] three being mixed, five happens. [works out examples 8, 13, 21] ... In this way, the process should be followed in all mātrā-vṛttas [prosodic combinations].

Hemachandra (c. 1150) is credited with knowledge of the sequence as well, writing that "the sum of the last and the one before the last is the number ... of the next mātrā-vṛtta."

### Europe

The Fibonacci sequence first appears in the book Liber Abaci (The Book of Calculation, 1202) by Fibonacci, where it is used to calculate the growth of rabbit populations. Fibonacci considers the growth of an idealized (biologically unrealistic) rabbit population, assuming that: a newly born breeding pair of rabbits are put in a field; each breeding pair mates at the age of one month, and at the end of their second month they always produce another pair of rabbits; and rabbits never die, but continue breeding forever. Fibonacci posed the rabbit math problem: how many pairs will there be in one year?

At the end of the first month, they mate, but there is still only 1 pair.

At the end of the second month they produce a new pair, so there are 2 pairs in the field.

At the end of the third month, the original pair produce a second pair, but the second pair only mate to gestate for a month, so there are 3 pairs in all.

At the end of the fourth month, the original pair has produced yet another new pair, and the pair born two months ago also produces their first pair, making 5 pairs.

At the end of the n-th month, the number of pairs of rabbits is equal to the number of mature pairs (that is, the number of pairs in month n – 2) plus the number of pairs alive last month (month n – 1). The number in the n-th month is the n-th Fibonacci number.

The name "Fibonacci sequence" was first used by the 19th-century number theorist Édouard Lucas.

### Closed-form expression

Like every sequence defined by a homogeneous linear recurrence with constant coefficients, the Fibonacci numbers have a closed-form expression. It has become known as Binet's formula, named after French mathematician Jacques Philippe Marie Binet, though it was already known by Abraham de Moivre and Daniel Bernoulli:

Fn=φn−ψnφ−ψ=φn−ψn5,

where ⁠φ⁠ (phi) is the golden ratio and ⁠ψ⁠ (psi) is its conjugate,

φ=12(1+5)=−1.61803…,ψ=12(1−5)=−0.61803….

The numbers ⁠φ⁠ and ⁠ψ⁠ are the two solutions of the quadratic equation ⁠x2−x−1=0⁠, that is, ⁠(x−φ)(x−ψ)=x2−x−1⁠, and thus they satisfy the identities ⁠φ+ψ=1⁠ and ⁠φψ=−1⁠.

Since ψ=−φ−1, Binet's formula can also be written as

Fn=φn−(−φ)−n5=φn−(−φ)−n2φ−1.

To see the relation between the sequence and these constants, note that φ and ψ are also roots of xn=xn−1+xn−2, so the powers of φ and ψ satisfy the Fibonacci recurrence. In other words,

φn=φn−1+φn−2,ψn=ψn−1+ψn−2.

It follows that for any values a and b, the sequence defined by

Un=aφn+bψn

satisfies the same recurrence. If a and b are chosen so that U0 = 0 and U1 = 1 then the resulting sequence Un must be the Fibonacci sequence. This is the same as requiring a and b satisfy the system of equations:

aφ0+bψ0=0aφ1+bψ1=1

which has solution

a=1φ−ψ=15,b=−a,

producing the required formula.

Taking the starting values U0 and U1 to be arbitrary constants and solving the system of equations gives the general solution

a=U1−U0ψ5,b=U0φ−U15.

In particular, choosing a = 1 makes the n-th element of the sequence closely approximate the n-th power of ⁠φ⁠ for large enough values of n. This arises when U0 = 2 and U1 = 1, which produces the sequence of Lucas numbers.

### Computation by rounding

Since

|ψn5|<12 for all n ≥ 0, the number Fn is the closest integer to φn5. Therefore, it can be found by rounding, using the nearest integer function:

Fn=⌊φn5⌉, n≥0.

In fact, the rounding error quickly becomes very small as n grows, being less than 0.1 for n ≥ 4, and less than 0.01 for n ≥ 8. This formula is easily inverted to find an index of a Fibonacci number F:

n(F)=⌊logφ5F⌉, F≥1.

Instead using the floor function gives the largest index of a Fibonacci number that is not greater than F:

nlargest(F)=⌊logφ5(F+1/2)⌋, F≥0,

where logφ(x)=ln⁡(x)/ln⁡(φ)=log10(x)/log10(φ), ln⁡(φ)=0.481211…, and log10(φ)=0.208987….

### Magnitude

Since Fn is asymptotic to φn/5, the number of digits in Fn is asymptotic to nlog10φ≈0.2090n. As a consequence, for every integer d > 1 there are either 4 or 5 Fibonacci numbers with d decimal digits.

More generally, in the base b representation, the number of digits in Fn is asymptotic to nlogbφ=nlog⁡φlog⁡b.

### Limit of consecutive quotients

Johannes Kepler observed that the ratio of consecutive Fibonacci numbers converges. He wrote that "as 5 is to 8 so is 8 to 13, practically, and as 8 is to 13, so is 13 to 21 almost", and concluded that these ratios approach the golden ratio ⁠φ⁠:

limn→∞Fn+1Fn=φ.

This convergence holds regardless of the starting values U0 and U1, unless U1=−U0/φ. This can be verified using Binet's formula. For example, the initial values 3 and 2 generate the sequence 3, 2, 5, 7, 12, 19, 31, 50, 81, 131, 212, 343, 555, ... . The ratio of consecutive elements in this sequence shows the same convergence towards the golden ratio.

In general, limn→∞Fn+mFn=φm, because the ratios between consecutive Fibonacci numbers approaches φ.

### Decomposition of powers

Since the golden ratio satisfies the equation

φ2=φ+1,

this expression can be used to decompose higher powers φn as a linear function of lower powers, which in turn can be decomposed all the way down to a linear combination of φ and 1. The resulting recurrence relationships yield Fibonacci numbers as the linear coefficients:

φn=Fnφ+Fn−1.

This equation can be proved by induction on n ≥ 1:

φn+1=(Fnφ+Fn−1)φ=Fnφ2+Fn−1φ=Fn(φ+1)+Fn−1φ=(Fn+Fn−1)φ+Fn=Fn+1φ+Fn.

For ψ=−1/φ, it is also the case that ψ2=ψ+1 and it is also the case that

ψn=Fnψ+Fn−1.

These expressions are also true for n < 1 if the Fibonacci sequence Fn is extended to negative integers using the Fibonacci rule Fn=Fn+2−Fn+1.

### Identification

A positive integer x is a Fibonacci number if and only if at least one of 5x2+4 or 5x2−4 is a perfect square.

A proof that x=Fn satisfies this property can be given using Binet's formula. Indeed, Fn=(φn−(−1)nφ−n)/5. View this as an equation in φn. After a simple computation, this becomes a quadratic equation in φn whose solution is:

φn=Fn5±5Fn2+4(−1)n2.

On the other hand, φn=Fnφ+Fn−1=(Fn5+Fn+2Fn−1)/2. It follows that:

5Fn2+4(−1)n=(Fn+2Fn−1)2.

In particular, the left-hand side is a perfect square.

The converse, namely that if x is such that one of 5x2+4 or 5x2−4 is a perfect square then x is a Fibonacci number (i.e. x=Fn for some n) is a little more complicated.

## Matrix form

A 2-dimensional system of linear difference equations that describes the Fibonacci sequence is

(Fk+2Fk+1)=(1110)(Fk+1Fk)

alternatively denoted

F→k+1=𝐀F→k,

which yields F→n=𝐀nF→0. The eigenvalues of the matrix A are φ=12(1+5) and ψ=−φ−1=12(1−5) corresponding to the respective eigenvectors

μ→=(φ1),ν→=(−φ−11).

As the initial value is

F→0=(10)=15μ→−15ν→,

it follows that the nth element is

F→n =15Anμ→−15Anν→=15φnμ→−15(−φ)−nν→=15(1+52)n(φ1)−15(1−52)n(−φ−11).

From this, the nth element in the Fibonacci sequence may be read off directly as a closed-form expression:

Fn=15(1+52)n−15(1−52)n.

Equivalently, the same computation may be performed by diagonalization of A through use of its eigendecomposition:

A=SΛS−1,An=SΛnS−1,

where

Λ=(φ00−φ−1),S=(φ−φ−111).

The closed-form expression for the nth element in the Fibonacci sequence is therefore given by

(Fn+1Fn)=An(F1F0) =SΛnS−1(F1F0)=S(φn00(−φ)−n)S−1(F1F0)=(φ−φ−111)(φn00(−φ)−n)15(1φ−1−1φ)(10),

which again yields

Fn=φn−(−φ)−n5.

The matrix A has a determinant of −1, and thus it is a 2 × 2 unimodular matrix.

This property can be understood in terms of the continued fraction representation for the golden ratio φ:

φ=1+11+11+11+⋱.

The convergents of the continued fraction for φ are ratios of successive Fibonacci numbers: φn = Fn+1 / Fn is the n-th convergent, and the (n + 1)-st convergent can be found from the recurrence relation φn+1 = 1 + 1 / φn. The matrix formed from successive convergents of any continued fraction has a determinant of +1 or −1. The matrix representation gives the following closed-form expression for the Fibonacci numbers:

(1110)n=(Fn+1FnFnFn−1).

For a given n, this matrix can be computed in O(log n) arithmetic operations, using the exponentiation by squaring method.

Taking the determinant of both sides of this equation yields Cassini's identity,

(−1)n=Fn+1Fn−1−Fn2.

Moreover, since AnAm = An+m for any square matrix A, the following identities can be derived (they are obtained from two different coefficients of the matrix product, and one may easily deduce the second one from the first one by changing n into n + 1),

FmFn+Fm−1Fn−1=Fm+n−1,FmFn+1+Fm−1Fn=Fm+n.

In particular, with m = n,

F2n−1=Fn2+Fn−12F2n−1=(Fn−1+Fn+1)Fn=(2Fn−1+Fn)Fn=(2Fn+1−Fn)Fn.

These last two identities provide a way to compute Fibonacci numbers recursively in O(log n) arithmetic operations. This matches the time for computing the n-th Fibonacci number from the closed-form matrix formula, but with fewer redundant steps if one avoids recomputing an already computed Fibonacci number (recursion with memoization).

### Combinatorial proofs

Most identities involving Fibonacci numbers can be proved using combinatorial arguments using the fact that Fn can be interpreted as the number of (possibly empty) sequences of 1s and 2s whose sum is n−1. This can be taken as the definition of Fn with the conventions F0=0, meaning no such sequence exists whose sum is −1, and F1=1, meaning the empty sequence "adds up" to 0. In the following, |...| is the cardinality of a set:

F0=0=|{}|

F1=1=|{()}|

F2=1=|{(1)}|

F3=2=|{(1,1),(2)}|

F4=3=|{(1,1,1),(1,2),(2,1)}|

F5=5=|{(1,1,1,1),(1,1,2),(1,2,1),(2,1,1),(2,2)}|

In this manner the recurrence relation

Fn=Fn−1+Fn−2

may be understood by dividing the Fn sequences into two non-overlapping sets where all sequences either begin with 1 or 2:

Fn=|{(1,...),(1,...),...}|+|{(2,...),(2,...),...}|

Excluding the first element, the remaining terms in each sequence sum to n−2 or n−3 and the cardinality of each set is Fn−1 or Fn−2 giving a total of Fn−1+Fn−2 sequences, showing this is equal to Fn.

In a similar manner it may be shown that the sum of the first Fibonacci numbers up to the n-th is equal to the (n + 2)-th Fibonacci number minus 1.   In symbols:

∑i=1nFi=Fn+2−1

This may be seen by dividing all sequences summing to n+1 based on the location of the first 2. Specifically, each set consists of those sequences that start (2,...),(1,2,...),..., until the last two sets {(1,1,...,1,2)},{(1,1,...,1)} each with cardinality 1.

Following the same logic as before, by summing the cardinality of each set we see that

Fn+2=Fn+Fn−1+...+|{(1,1,...,1,2)}|+|{(1,1,...,1)}|

... where the last two terms have the value F1=1. From this it follows that ∑i=1nFi=Fn+2−1.

A similar argument, grouping the sums by the position of the first 1 rather than the first 2 gives two more identities:

∑i=0n−1F2i+1=F2n

and

∑i=1nF2i=F2n+1−1.

In words, the sum of the first Fibonacci numbers with odd index up to F2n−1 is the (2n)-th Fibonacci number, and the sum of the first Fibonacci numbers with even index up to F2n is the (2n + 1)-th Fibonacci number minus 1.

A different trick may be used to prove

∑i=1nFi2=FnFn+1

or in words, the sum of the squares of the first Fibonacci numbers up to Fn is the product of the n-th and (n + 1)-th Fibonacci numbers. To see this, begin with a Fibonacci rectangle of size Fn×Fn+1 and decompose it into squares of size Fn,Fn−1,...,F1; from this the identity follows by comparing areas:

### Induction proofs

Fibonacci identities often can be easily proved using mathematical induction.

For example, reconsider

∑i=1nFi=Fn+2−1.

Adding Fn+1 to both sides gives

∑i=1nFi+Fn+1=Fn+1+Fn+2−1

and so we have the formula for n+1

∑i=1n+1Fi=Fn+3−1

Similarly, add Fn+12 to both sides of

∑i=1nFi2=FnFn+1

to give

∑i=1nFi2+Fn+12=Fn+1(Fn+Fn+1)

∑i=1n+1Fi2=Fn+1Fn+2

### Binet formula proofs

The Binet formula

5Fn=φn−ψn

can be used to prove Fibonacci identities.

For example, to prove that ∑i=1nFi=Fn+2−1,

multiply both sides by 5 and note that the left hand side  becomes

1+φ+φ2+⋯+φn−(1+ψ+ψ2+⋯+ψn)=φn+1−1φ−1−ψn+1−1ψ−1=φn+2−φφ2−φ−ψn+2−ψψ2−ψ=φn+2−ψn+2−(φ−ψ)=5(Fn+2−1)

as required, using the facts that φ2−φ=ψ2−ψ=1 and φ−ψ=5F1=5 to simplify the equations.

## Other identities

Numerous other identities can be derived using various methods. Here are some of them:

### Cassini's and Catalan's identities

Cassini's identity states that

Fn2−Fn+1Fn−1=(−1)n−1

Catalan's identity is a generalization:

Fn2−Fn+rFn−r=(−1)n−rFr2

### d'Ocagne's identity

FmFn+1−Fm+1Fn=(−1)nFm−n

F2n=Fn+12−Fn−12=Fn(Fn+1+Fn−1)=FnLn

where Ln is the n-th Lucas number. The last is an identity for doubling n; other identities of this type are

F3n=2Fn3+3FnFn+1Fn−1=5Fn3+3(−1)nFn

by Cassini's identity.

F3n+1=Fn+13+3Fn+1Fn2−Fn3

F3n+2=Fn+13+3Fn+12Fn+Fn3

F4n=4FnFn+1(Fn+12+2Fn2)−3Fn2(Fn2+2Fn+12)

These can be found experimentally using lattice reduction, and are useful in setting up the special number field sieve to factorize a Fibonacci number.

More generally,

Fkn+c=∑i=0k(ki)Fc−iFniFn+1k−i=∑i=0k(ki)Fc+iFniFn−1k−i.

Putting k = 2 in this formula, one gets again the formulas of the end of § Matrix form.

### Ordinary

The ordinary generating function of the Fibonacci sequence is the power series

s(z)=∑k=0∞Fkzk=0+z+z2+2z3+3z4+5z5+⋯.

This series is convergent for any complex number z satisfying |z|<1/φ≈0.618, and its sum has a simple closed form:

s(z)=z1−z−z2.

This can be proved by multiplying by (1−z−z2):

(1−z−z2)s(z)=∑k=0∞Fkzk−∑k=0∞Fkzk+1−∑k=0∞Fkzk+2=∑k=0∞Fkzk−∑k=1∞Fk−1zk−∑k=2∞Fk−2zk=0z0+1z1−0z1+∑k=2∞(Fk−Fk−1−Fk−2)zk=z,

where all terms involving zk for k≥2 cancel out because of the defining Fibonacci recurrence relation.

Using z=10−n lays out the Fibonacci numbers through the second-last number with n digits in the decimal expansion of s(z). For example, s(10−3)=0.0010.998999=1000998999=000.001001002003005008013….

The partial fraction decomposition is given by

s(z)=15(11−φz−11−ψz)

where φ=12(1+5) is the golden ratio and ψ=12(1−5) is its conjugate.

### Exponential

The exponential generating function of the Fibonacci sequence may also be derived from the recurrence relation, giving a homogeneous linear differential equation:

∑k=0∞Fk+2xkk!=∑k=0∞Fk+1xkk!+∑k=0∞Fkxkk!F″(x)=F′(x)+F(x)

The characteristic polynomial of this equation is r2=r+1, to which the solutions are exactly the golden ratio φ and its conjugate ψ. Combined with the initial values F0=F(0)=0 and F1=F′(0)=1, the exponential generating function of the Fibonacci numbers is given by the entire function

F(x)=eφx−eψx5

Evaluating the derivatives of the exponential generating function at x=0 gives Binet's formula:

F(n)(0)=Fn=φn−ψn5

## Reciprocal sums

Infinite sums over reciprocal Fibonacci numbers can sometimes be evaluated in terms of theta functions. For example, the sum of every odd-indexed reciprocal Fibonacci number can be written as

∑k=1∞1F2k−1=54ϑ2(0,3−52)2,

and the sum of squared reciprocal Fibonacci numbers as

∑k=1∞1Fk2=524(ϑ2(0,3−52)4−ϑ4(0,3−52)4+1).

If we add 1 to each Fibonacci number in the first sum, there is also the closed form

∑k=1∞11+F2k−1=52,

and there is a nested sum of squared Fibonacci numbers giving the reciprocal of the golden ratio,

∑k=1∞(−1)k+1∑j=1kFj2=5−12.

The sum of all even-indexed reciprocal Fibonacci numbers is

∑k=1∞1F2k=5(L(ψ2)−L(ψ4))

with the Lambert series L(q):=∑k=1∞qk1−qk, since 1F2k=5(ψ2k1−ψ2k−ψ4k1−ψ4k).

So the reciprocal Fibonacci constant is

∑k=1∞1Fk=∑k=1∞1F2k−1+∑k=1∞1F2k=3.359885666243…

Moreover, this number has been proved irrational by Richard André-Jeannin.

Millin's series gives the identity

∑k=0∞1F2k=7−52,

which follows from the closed form for its partial sums as N tends to infinity:

∑k=0N1F2k=3−F2N−1F2N.

### Divisibility properties

Every third number of the sequence is even (a multiple of ⁠F3=2⁠) and, more generally, every ⁠k⁠-th number of the sequence is a multiple of ⁠Fk⁠. Thus the Fibonacci sequence is an example of a divisibility sequence. In fact, the Fibonacci sequence satisfies the stronger divisibility property

gcd⁡(Fa,Fb,Fc,…)=Fgcd⁡(a,b,c,…)

where gcd is the greatest common divisor function. (This relation is different if a different indexing convention is used, such as the one that starts the sequence with ⁠F0=1⁠ and ⁠F1=1⁠.)

In particular, any three consecutive Fibonacci numbers are pairwise coprime because both F1=1 and ⁠F2=1⁠. That is,

gcd⁡(Fn,Fn+1)=gcd⁡(Fn,Fn+2)=gcd⁡(Fn+1,Fn+2)=1

for every n.

Every prime number p divides a Fibonacci number that can be determined by the value of p modulo 5. If p is congruent to 1 or 4 modulo 5, then p divides Fp−1, and if p is congruent to 2 or 3 modulo 5, then, p divides Fp+1. The remaining case is that p = 5, and in this case p divides Fp.

{p=5⇒p∣Fp,p≡±1(mod5)⇒p∣Fp−1,p≡±2(mod5)⇒p∣Fp+1.

These cases can be combined into a single, non-piecewise formula, using the Legendre symbol:

p∣Fp−(5p).

### Primality testing

The above formula can be used as a primality test in the sense that if

n∣Fn−(5n),

where the Legendre symbol has been replaced by the Jacobi symbol, then this is evidence that n is a prime, and if it fails to hold, then n is definitely not a prime. If n is composite and satisfies the formula, then n is a Fibonacci pseudoprime. When m is large – say a 500-bit number – then we can calculate Fm (mod n) efficiently using the matrix form. Thus

(Fm+1FmFmFm−1)≡(1110)m(modn).

Here the matrix power Am is calculated using modular exponentiation, which can be adapted to matrices.

### Fibonacci primes

A Fibonacci prime is a Fibonacci number that is prime. The first few are:

2, 3, 5, 13, 89, 233, 1597, 28657, 514229, ...

Fibonacci primes with thousands of digits have been found, but it is not known whether there are infinitely many.

Fkn is divisible by Fn, so, apart from F4 = 3, any Fibonacci prime must have a prime index. As there are arbitrarily long runs of composite numbers, there are therefore also arbitrarily long runs of composite Fibonacci numbers.

No Fibonacci number greater than F6 = 8 is one greater or one less than a prime number.

The only nontrivial square Fibonacci number is 144.  Attila Pethő proved in 2001 that there is only a finite number of perfect power Fibonacci numbers. In 2006, Y. Bugeaud, M. Mignotte, and S. Siksek proved that 8 and 144 are the only such non-trivial perfect powers.

The only triangular Fibonacci numbers are 1, 3, 21, and 55, which was conjectured by Vern Hoggatt and proved by Luo Ming.

No Fibonacci number can be a perfect number. More generally, no Fibonacci number other than 1 can be multiply perfect, and no ratio of two Fibonacci numbers can be perfect.

### Prime divisors

With the exceptions of 1, 8 and 144 (F1 = F2, F6 and F12) every Fibonacci number has a prime factor that is not a factor of any smaller Fibonacci number (Carmichael's theorem). As a result, 8 and 144 (F6 and F12) are the only Fibonacci numbers that are the product of other Fibonacci numbers.

The divisibility of Fibonacci numbers by a prime p is related to the Legendre symbol (p5) which is evaluated as follows:

(p5)={0if p=51if p≡±1(mod5)−1if p≡±2(mod5).

If p is a prime number then

Fp≡(p5)(modp)andFp−(p5)≡0(modp).

For example,

(25)=−1,F3=2,F2=1,(35)=−1,F4=3,F3=2,(55)=0,F5=5,(75)=−1,F8=21,F7=13,(115)=+1,F10=55,F11=89.

It is not known whether there exists a prime p such that

Fp−(p5)≡0(modp2).

Such primes (if there are any) would be called Wall–Sun–Sun primes.

Also, if p ≠ 5 is an odd prime number then:

5Fp±122≡{12(5(p5)±5)(modp)if p≡1(mod4)12(5(p5)∓3)(modp)if p≡3(mod4).

Example 1. p = 7, in this case p ≡ 3 (mod 4) and we have:

(75)=−1:12(5(75)+3)=−1,12(5(75)−3)=−4.

F3=2 and F4=3.

5F32=20≡−1(mod7) and 5F42=45≡−4(mod7)

Example 2. p = 11, in this case p ≡ 3 (mod 4) and we have:

(115)=+1:12(5(115)+3)=4,12(5(115)−3)=1.

F5=5 and F6=8.

5F52=125≡4(mod11) and 5F62=320≡1(mod11)

Example 3. p = 13, in this case p ≡ 1 (mod 4) and we have:

(135)=−1:12(5(135)−5)=−5,12(5(135)+5)=0.

F6=8 and F7=13.

5F62=320≡−5(mod13) and 5F72=845≡0(mod13)

Example 4. p = 29, in this case p ≡ 1 (mod 4) and we have:

(295)=+1:12(5(295)−5)=0,12(5(295)+5)=5.

F14=377 and F15=610.

5F142=710645≡0(mod29) and 5F152=1860500≡5(mod29)

For odd n, all odd prime divisors of Fn are congruent to 1 modulo 4, implying that all odd divisors of Fn (as the products of odd prime divisors) are congruent to 1 modulo 4.

For example,

F1=1, F3=2, F5=5, F7=13, F9=34=2⋅17, F11=89, F13=233, F15=610=2⋅5⋅61.

All known factors of Fibonacci numbers F(i) for all i < 50000 are collected at the relevant repositories.

### Periodicity modulo n

If the members of the Fibonacci sequence are taken mod n, the resulting sequence is periodic with period at most 6n. The lengths of the periods for various n form the so-called Pisano periods. Determining a general formula for the Pisano periods is an open problem, which includes as a subproblem a special instance of the problem of finding the multiplicative order of a modular integer or of an element in a finite field. However, for any particular n, the Pisano period may be found as an instance of cycle detection.

## Generalizations

The Fibonacci sequence is one of the simplest and earliest known sequences defined by a recurrence relation, and specifically by a linear difference equation. All these sequences may be viewed as generalizations of the Fibonacci sequence. In particular, Binet's formula may be generalized to any sequence that is a solution of a homogeneous linear difference equation with constant coefficients.

Some specific examples that are close, in some sense, to the Fibonacci sequence include:

Generalizing the index to negative integers to produce the negafibonacci numbers.

Generalizing the index to real numbers using a modification of Binet's formula.

Starting with other integers. Lucas numbers have L1 = 1, L2 = 3, and Ln = Ln−1 + Ln−2. Primefree sequences use the Fibonacci recursion with other starting points to generate sequences in which all numbers are composite.

Letting a number be a linear function (other than the sum) of the 2 preceding numbers. The Pell numbers have Pn = 2Pn−1 + Pn−2. If the coefficient of the preceding value is assigned a variable value x, the result is the sequence of Fibonacci polynomials.

Not adding the immediately preceding numbers. The Padovan sequence and Perrin numbers have P(n) = P(n − 2) + P(n − 3).

Generating the next number by adding 3 numbers (tribonacci numbers), 4 numbers (tetranacci numbers), or more. The resulting sequences are known as k-Step Fibonacci numbers. They are also commonly referred to as k-bonacci numbers.

### Mathematics

The Fibonacci numbers occur as the sums of binomial coefficients in the "shallow" diagonals of Pascal's triangle:

Fn=∑k=0⌊n−12⌋(n−k−1k).

This can be proved by expanding the generating function

x1−x−x2=x+x2(1+x)+x3(1+x)2+⋯+xk+1(1+x)k+⋯=∑n=0∞Fnxn

and collecting like terms of xn.

To see how the formula is used, we can arrange the sums by the number of terms present:

which is (50)+(41)+(32), where we are choosing the positions of k twos from n−k−1 terms.

These numbers also give the solution to certain enumerative problems, the most common of which is that of counting the number of ways of writing a given number n as an ordered sum of 1s and 2s (called compositions); there are Fn+1 ways to do this (equivalently, it's also the number of domino tilings of the 2×n rectangle). For example, there are F5+1 = F6 = 8 ways one can climb a staircase of 5 steps, taking one or two steps at a time:

The figure shows that 8 can be decomposed into 5 (the number of ways to climb 4 steps, followed by a single-step) plus 3 (the number of ways to climb 3 steps, followed by a double-step). The same reasoning is applied recursively until a single step, of which there is only one way to climb.

The Fibonacci numbers can be found in different ways among the set of binary strings, or equivalently, among the subsets of a given set.

The number of binary strings of length n without consecutive 1s is the Fibonacci number Fn+2. For example, out of the 16 binary strings of length 4, there are F6 = 8 without consecutive 1s—they are 0000, 0001, 0010, 0100, 0101, 1000, 1001, and 1010. Such strings are the binary representations of Fibbinary numbers. Equivalently, Fn+2 is the number of subsets S of {1, ..., n} without consecutive integers, that is, those S for which {i, i + 1} ⊈ S for every i. A bijection with the sums to n+1 is to replace 1 with 0 and 2 with 10, and drop the last zero.

The number of binary strings of length n without an odd number of consecutive 1s is the Fibonacci number Fn+1. For example, out of the 16 binary strings of length 4, there are F5 = 5 without an odd number of consecutive 1s—they are 0000, 0011, 0110, 1100, 1111. Equivalently, the number of subsets S of {1, ..., n} without an odd number of consecutive integers is Fn+1. A bijection with the sums to n is to replace 1 with 0 and 2 with 11.

The number of binary strings of length n without an even number of consecutive 0s or 1s is 2Fn. For example, out of the 16 binary strings of length 4, there are 2F4 = 6 without an even number of consecutive 0s or 1s—they are 0001, 0111, 0101, 1000, 1010, 1110. There is an equivalent statement about subsets.

Yuri Matiyasevich was able to show that the Fibonacci numbers can be defined by a Diophantine equation, which led to his solving Hilbert's tenth problem.

The Fibonacci numbers are also an example of a complete sequence. This means that every positive integer can be written as a sum of Fibonacci numbers, where any one number is used once at most.

Moreover, every positive integer can be written in a unique way as the sum of one or more distinct Fibonacci numbers in such a way that the sum does not include any two consecutive Fibonacci numbers. This is known as Zeckendorf's theorem, and a sum of Fibonacci numbers that satisfies these conditions is called a Zeckendorf representation. The Zeckendorf representation of a number can be used to derive its Fibonacci coding.

Starting with 5, every second Fibonacci number is the length of the hypotenuse of a right triangle with integer sides, or in other words, the largest number in a Pythagorean triple, obtained from the formula (FnFn+3)2+(2Fn+1Fn+2)2=F2n+32. The sequence of Pythagorean triangles obtained from this formula has sides of lengths (3,4,5), (5,12,13), (16,30,34), (39,80,89), ... . The middle side of each of these triangles is the sum of the three sides of the preceding triangle.

The Fibonacci cube is an undirected graph with a Fibonacci number of nodes that has been proposed as a network topology for parallel computing.

Fibonacci numbers appear in the ring lemma, used to prove connections between the circle packing theorem and conformal maps.

### Computer science

The Fibonacci numbers are important in computational run-time analysis of Euclid's algorithm to determine the greatest common divisor of two integers: the worst case input for this algorithm is a pair of consecutive Fibonacci numbers.

Fibonacci numbers are used in a polyphase version of the merge sort algorithm in which an unsorted list is divided into two lists whose lengths correspond to sequential Fibonacci numbers—by dividing the list so that the two parts have lengths in the approximate proportion φ. A tape-drive implementation of the polyphase merge sort was described in The Art of Computer Programming.

A Fibonacci tree is a binary tree whose child trees (recursively) differ in height by exactly 1. So it is an AVL tree, and one with the fewest nodes for a given height—the "thinnest" AVL tree. These trees have a number of vertices that is a Fibonacci number minus one, an important fact in the analysis of AVL trees.

Fibonacci numbers are used by some pseudorandom number generators.

Fibonacci numbers arise in the analysis of the Fibonacci heap data structure.

A one-dimensional optimization method, called the Fibonacci search technique, uses Fibonacci numbers.

The Fibonacci number series is used for optional lossy compression in the IFF 8SVX audio file format used on Amiga computers. The number series compands the original audio wave similar to logarithmic methods such as μ-law.

Some Agile teams use a modified series called the "Modified Fibonacci Series" in planning poker, as an estimation tool. Planning Poker is a formal part of the Scaled Agile Framework.

Fibonacci coding

Negafibonacci coding

In topological quantum computing, quasi-particles called Fibonacci anyons could be used for universal quantum computation. The dimension of the Hilbert space describing n Fibonacci anyons is the Fibonacci number Fn+1.

### Nature

Fibonacci sequences appear in biological settings, such as branching in trees, arrangement of leaves on a stem, the fruitlets of a pineapple, the flowering of artichoke, the leaves of the spiral aloe (Aloe polyphylla), the arrangement of a pine cone, and the family tree of honeybees. Kepler pointed out the presence of the Fibonacci sequence in nature, using it to explain the (golden ratio-related) pentagonal form of some flowers. Field daisies most often have petals in counts of Fibonacci numbers. In 1830, Karl Friedrich Schimper and Alexander Braun discovered that the parastichies (spiral phyllotaxis) of plants were frequently expressed as fractions involving Fibonacci numbers.

Przemysław Prusinkiewicz advanced the idea that real instances can in part be understood as the expression of certain algebraic constraints on free groups, specifically as certain Lindenmayer grammars.

A model for the pattern of florets in the head of a sunflower was proposed by Helmut Vogel in 1979.  This has the form

θ=2πφ2n, r=cn

where n is the index number of the floret and c is a constant scaling factor; the florets thus lie on Fermat's spiral. The divergence angle, approximately 137.51°, is the golden angle, dividing the circle in the golden ratio. Because this ratio is irrational, no floret has a neighbor at exactly the same angle from the center, so the florets pack efficiently. Because the rational approximations to the golden ratio are of the form F( j):F( j + 1), the nearest neighbors of floret number n are those at n ± F( j) for some index j, which depends on r, the distance from the center. Sunflowers and similar flowers most commonly have spirals of florets in clockwise and counter-clockwise directions in the amount of adjacent Fibonacci numbers, typically counted by the outermost range of radii.

Fibonacci numbers also appear in the ancestral pedigrees of bees (which are haplodiploids), according to the following rules:

If an egg is laid but not fertilized, it produces a male (or drone bee in honeybees).

If, however, an egg is fertilized, it produces a female.

Thus, a male bee always has one parent, and a female bee has two. If one traces the pedigree of any male bee (1 bee), he has 1 parent (1 bee), 2 grandparents, 3 great-grandparents, 5 great-great-grandparents, and so on. This sequence of numbers of parents is the Fibonacci sequence. The number of ancestors at each level, Fn, is the number of female ancestors, which is Fn−1, plus the number of male ancestors, which is Fn−2. This is under the unrealistic assumption that the ancestors at each level are otherwise unrelated.

It has similarly been noticed that the number of possible ancestors on the human X chromosome inheritance line at a given ancestral generation also follows the Fibonacci sequence. A male individual has an X chromosome, which he received from his mother, and a Y chromosome, which he received from his father. The male counts as the "origin" of his own X chromosome (F1=1), and at his parents' generation, his X chromosome came from a single parent (F2=1). The male's mother received one X chromosome from her mother (the son's maternal grandmother), and one from her father (the son's maternal grandfather), so two grandparents contributed to the male descendant's X chromosome (F3=2). The maternal grandfather received his X chromosome from his mother, and the maternal grandmother received X chromosomes from both of her parents, so three great-grandparents contributed to the male descendant's X chromosome (F4=3). Five great-great-grandparents contributed to the male descendant's X chromosome (F5=5), etc. (This assumes that all ancestors of a given descendant are independent, but if any genealogy is traced far enough back in time, ancestors begin to appear on multiple lines of the genealogy, until eventually a population founder appears on all lines of the genealogy.)

### Other

In optics, when a beam of light shines at an angle through two stacked transparent plates of different materials of different refractive indexes, it may reflect off three surfaces: the top, middle, and bottom surfaces of the two plates. The number of different beam paths that have k reflections, for k > 1, is the k-th Fibonacci number. (However, when k = 1, there are three reflection paths, not two, one for each of the three surfaces.)

Fibonacci retracement levels are widely used in technical analysis for financial market trading.

Since the conversion factor 1.609344 for miles to kilometers is close to the golden ratio, the decomposition of distance in miles into a sum of Fibonacci numbers becomes nearly the kilometer sum when the Fibonacci numbers are replaced by their successors. This method amounts to a radix 2 number register in golden ratio base φ being shifted. To convert from kilometers to miles, shift the register down the Fibonacci sequence instead.

The measured values of voltages and currents in the infinite resistor chain circuit (also called the resistor ladder or infinite series-parallel circuit) follow the Fibonacci sequence. The intermediate results of adding the alternating series and parallel resistances yields fractions composed of consecutive Fibonacci numbers. The equivalent resistance of the entire circuit equals the golden ratio.

Brasch et al. 2012 show how a generalized Fibonacci sequence also can be connected to the field of economics. In particular, it is shown how a generalized Fibonacci sequence enters the control function of finite-horizon dynamic optimisation problems with one state and one control variable. The procedure is illustrated in an example often referred to as the Brock–Mirman economic growth model.

Mario Merz included the Fibonacci sequence in some of his artworks beginning in 1970.

Joseph Schillinger (1895–1943) developed a system of composition which uses Fibonacci intervals in some of its melodies; he viewed these as the musical counterpart to the elaborate harmony evident within nature. See also Golden ratio § Music.

In software development, Fibonacci numbers are often used by agile teams operating under the Scrum framework to size their product backlog items.
