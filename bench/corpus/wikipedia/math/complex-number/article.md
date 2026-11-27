# Complex number

In mathematics, a complex number is a number of the form ⁠a+bi⁠, where a and b are real numbers, and i is the imaginary unit, a number that satisfies the equation ⁠i2=−1⁠; the number a is called the real part of ⁠a+bi⁠, and b is called the imaginary part.

The complex numbers form a number system which extends the real numbers and supports the same basic arithmetic operations of addition, subtraction, multiplication, and division, satisfying the same associative, commutative, and distributive laws. This makes the complex numbers a field. The set of complex numbers is denoted by either of the symbols ⁠ℂ⁠ or C.

Because any nonzero real number, whether positive or negative, squares to a positive number, taking the square root of a negative number was historically considered impossible. When, in the 16th century, such square roots were found helpful for determining the real solutions of cubic equations, they were originally considered a mere symbolic contrivance, and in the 17th century were given the name imaginary numbers. Despite the historical nomenclature, "imaginary" numbers are not fictitious: they are no less valid mathematically than real numbers, and they are essential to the scientific description of the physical world.

Complex numbers allow solutions to all non-constant polynomial equations with real or complex coefficients, even those that have no solutions in real numbers, a result known as the fundamental theorem of algebra. The complex numbers also form a real vector space of dimension two, with {1,i} as a standard basis. Thus each complex number z=a+ib can be uniquely associated to a point with coordinates (a,b) in a Cartesian plane, called the complex plane.  This allows a geometric interpretation of the complex numbers and their operations; in particular, addition by a complex number corresponds to translation of the plane and multiplication by a complex number corresponds to scaling and rotation. Conversely some geometric objects and operations can be expressed in terms of complex numbers, enabling the solution of geometric problems using algebraic methods.

## Definition and basic operations

A complex number is an expression of the form a + bi, where a and b are real numbers, and i is an abstract symbol, the imaginary unit, whose meaning will be explained further below. For example, 2 + 3i is a complex number.

For a complex number a + bi, the real number a is its real part, and the real number b (not the complex number bi) is its imaginary part. The real part of a complex number z is denoted Re(z), ⁠ℛℯ(z)⁠, or ⁠ℜ(z)⁠; the imaginary part is Im(z), ⁠ℐ𝓂(z)⁠, or ⁠ℑ(z)⁠: for example, ⁠Re⁡(2+3i)=2⁠, ⁠Im⁡(2+3i)=3⁠.

A complex number z can be identified with the ordered pair of real numbers ⁠(ℜ(z),ℑ(z))⁠, which may be interpreted as coordinates of a point in a Euclidean plane with standard coordinates, which is the complex plane or Argand diagram. The horizontal axis is generally used to display the real part, with increasing values to the right, and the imaginary part marks the vertical axis, with increasing values upwards.

A real number a can be regarded as a complex number a + 0i, whose imaginary part is 0. A purely imaginary number bi is a complex number 0 + bi, whose real part is zero. It is common to write a + 0i = a, 0 + bi = bi, and a + (−b)i = a − bi; for example, 3 + (−4)i = 3 − 4i.

The set of all complex numbers is denoted by ⁠ℂ⁠ (blackboard bold) or C (upright bold).

In some disciplines such as electromagnetism and electrical engineering, j is used instead of i, as i frequently represents electric current, and complex numbers are written as a + bj or a + jb.

### Addition, additive inverse, and subtraction

Two complex numbers a=x+yi and b=u+vi are added by separately adding their real and imaginary parts. That is to say:

a+b=(x+yi)+(u+vi)=(x+u)+(y+v)i.

In particular, the additive inverse of a = x + yi must be −a = (−x) + (−y)i = −x − yi because a + (−a) gives zero by the above addition formula.

Similarly, subtraction can be performed as

a−b=(x+yi)−(u+vi)=(x−u)+(y−v)i.

The addition can be geometrically visualized as follows: the sum of two complex numbers a and b, interpreted as points in the complex plane, is the point obtained by building a parallelogram from the three vertices O, and the points of the arrows labeled a and b (provided that they are not on a line). Equivalently, calling these points A, B, respectively and the fourth point of the parallelogram X the triangles OAB and XBA are congruent.

### Multiplication

The product of two complex numbers can expanded using the distributive property and the identity ⁠i2=−1⁠:

(a+bi)⋅(c+di)=ac+adi+bci+bdi2=(ac−bd)+(ad+bc)i.

For example,

(2−i)⋅(3+4i)=(2⋅3−(−1)⋅4)+(2⋅4+(−1)⋅3)i=10+5i.

Using the sum rule from the previous section and this product rule, the familiar rules for the arithmetic of rational or real numbers continue to hold for complex numbers. Namely, addition and multiplication always satisfy the associative and commutative properties, and multiplication is distributive over addition.

### Complex conjugate, absolute value, argument and division

The complex conjugate of the complex number z = x + yi is defined as

⁠z‾=x−yi⁠. It is also denoted by some authors by ⁠z∗⁠. Geometrically, z is the "reflection" of z about the real axis. Conjugating twice gives the original complex number: z‾‾=z. A complex number is real if and only if it equals its own conjugate. The unary operation of taking the complex conjugate of a complex number cannot be expressed by applying only the basic operations of addition, subtraction, multiplication and division.

For any complex number z = x + yi , the product

z⋅z‾=(x+iy)(x−iy)=x2+y2

is a non-negative real number. This allows to define the absolute value (or modulus or magnitude) of z to be the square root

|z|=x2+y2.

By Pythagoras' theorem, |z| is the distance from the origin to the point representing the complex number z in the complex plane. In particular, the circle of radius one around the origin consists precisely of the numbers z such that ⁠|z|=1⁠, known as the unit complex numbers. If z=x=x+0i is a real number, then ⁠|z|=|x|⁠: its absolute value as a complex number and as a real number are equal.

Using the conjugate, the reciprocal of a nonzero complex number z=x+yi can be computed to be

1z=z¯zz¯=z¯|z|2=x−yix2+y2=xx2+y2−yx2+y2i.

Every complex number ⁠z⁠ has an additive inverse ⁠−z⁠ and every nonzero complex number ⁠z⁠ has a multiplicative inverse ⁠1/z⁠; the complex numbers form a field, much as the rational or real numbers do.

More generally, the division of an arbitrary complex number w=u+vi by a non-zero complex number z=x+yi equals

wz=wz¯|z|2=(u+vi)(x−iy)x2+y2=ux+vyx2+y2+vx−uyx2+y2i.

This process is sometimes called "rationalization" of the denominator (although the denominator in the final expression may be an irrational real number), because it resembles the method to remove roots from simple expressions in a denominator.

The argument of z, sometimes called the "phase", is the angle of the radius Oz measured anticlockwise from the positive real axis. It is written as arg z and is expressed in radians in this article. The angle is defined only up to adding integer multiples of ⁠2π⁠, since a rotation by 2π (or 360°) around the origin leaves all points in the complex plane unchanged. One possible choice to uniquely specify the argument (also known as choosing a principal value) is to require it to be within the interval ⁠(−π,π]⁠, though there are other possibilities.

The argument plays a role with complex logarithms; see below. It can be computed from the rectangular form x + yi by means of the arctan (inverse tangent) function.

### Polar form

For any complex number z, with absolute value r=|z| and argument ⁠φ=arg⁡z⁠, the equation

z=r(cos⁡φ+isin⁡φ)

holds. This identity is referred to as the polar form of z. It is sometimes written as ⁠z=rcis⁡φ⁠ or ⁠z=rexp⁡φi⁠.

In electronics, one represents a phasor with amplitude r and phase φ in angle notation:z=r∠φ.

If two complex numbers are given in polar form, i.e., z1 = r1(cos φ1 + i sin φ1) and z2 = r2(cos φ2 + i sin φ2), the product and division can be computed as

z1z2=r1r2(cos⁡(φ1+φ2)+isin⁡(φ1+φ2)).

z1z2=r1r2(cos⁡(φ1−φ2)+isin⁡(φ1−φ2)),if z2≠0.

(These are a consequence of the trigonometric identities for the sine and cosine function.)

In other words, the absolute values are multiplied and the arguments are added to yield the polar form of the product. The picture at the right illustrates the multiplication of

(2+i)(3+i)=5+5i.

Because the real and imaginary part of 5 + 5i are equal, the argument of that number is 45 degrees, or π/4 (in radian). On the other hand, it is also the sum of the angles at the origin of the red and blue triangles are arctan(1/3) and arctan(1/2), respectively. Thus, the formula

π4=arctan⁡(12)+arctan⁡(13)

holds. As the arctan function can be approximated highly efficiently, formulas like this – known as Machin-like formulas – are used for high-precision approximations of π:

π4=4arctan⁡(15)−arctan⁡(1239)

### Powers and roots

When n is an integer, the n-th power of a complex number can be computed using de Moivre's formula, which is obtained by repeatedly applying the above formula for the product:

zn=z⋅⋯⋅z⏟n factors=(r(cos⁡φ+isin⁡φ))n=rn(cos⁡nφ+isin⁡nφ).

For example, the first few powers of the imaginary unit i are ⁠i,i2=−1,i3=−i,i4=1,i5=i,…⁠.

When n is a positive integer, the n n-th roots of a complex number z are given by

z1/n=rn(cos⁡(φ+2kπn)+isin⁡(φ+2kπn))

for integer k satisfying 0 ≤ k ≤ n − 1 and where rn is the usual (positive) n-th root of the positive real number r. Because sine and cosine are periodic, other integer values of k do not give additional values. For any ⁠z≠0⁠, these n complex n-th roots are distinct. For example, there are 4 fourth roots of 1, namely

z1=1,z2=i,z3=−1,z4=−i.

For ⁠n≥2⁠, the n-th root is a multivalued function (or specifically, an n-valued function) of z.  One of these values can be designated as the principal value, typically via the technique of branch cuts of the complex plane.

### Fundamental theorem of algebra

The fundamental theorem of algebra, of Carl Friedrich Gauss and Jean le Rond d'Alembert, states that for any coefficients that are complex numbers a0, ..., an, the equation

anzn+⋯+a1z+a0=0

has at least one complex solution z, provided that at least one of the higher coefficients a1, ..., an is nonzero. This property does not hold for the field of rational numbers ⁠ℚ⁠ (the polynomial x2 − 2 does not have a rational root, because √2 is not a rational number) nor the real numbers ⁠ℝ⁠ (the polynomial x2 + 4 does not have a real root, because the square of x is positive for any real number x).

Because of this fact, ⁠ℂ⁠ is an algebraically closed field. It is a cornerstone of various applications of complex numbers, as is detailed further below.

There are various proofs of this theorem, by either analytic methods such as Liouville's theorem, or topological ones such as the winding number, or a proof combining Galois theory and the fact that any real polynomial of odd degree has at least one real root.

The field of complex numbers is defined as the (unique) algebraic extension field of the real numbers later in #Abstract algebraic definitions.

## History

The solution in radicals (without trigonometric functions) of a general cubic equation, when all three of its roots are real numbers, contains the square roots of negative numbers, a situation that cannot be rectified by factoring aided by the rational root test, if the cubic is irreducible; this is the casus irreducibilis ('irreducible case'). This conundrum led Italian mathematician Gerolamo Cardano to conceive of complex numbers around 1545 in his Ars Magna, though his understanding was rudimentary; moreover, he later described complex numbers as being "as subtle as they are useless". Cardano did use imaginary numbers, but described using them as "mental torture". This was prior to the use of the graphical complex plane. Cardano and other Italian mathematicians, notably Scipione del Ferro, in the 1500s created an algorithm for solving cubic equations which generally had one real solution and two solutions containing an imaginary number. Because they ignored the answers with the imaginary numbers, Cardano found them useless.

Work on the problem of general polynomials ultimately led to the fundamental theorem of algebra, which shows that with complex numbers, a solution exists to every polynomial equation of degree one or higher. Complex numbers thus form an algebraically closed field, where any polynomial equation has a root.

Many mathematicians contributed to the development of complex numbers. The rules for addition, subtraction, multiplication, and root extraction of complex numbers were developed by the Italian mathematician Rafael Bombelli. A more abstract formalism for the complex numbers was further developed by the Irish mathematician William Rowan Hamilton, who extended this abstraction to the theory of quaternions.

The earliest fleeting reference to square roots of negative numbers can perhaps be said to occur in the work of the Greek mathematician Hero of Alexandria in the 1st century AD, where in his Stereometrica he considered, apparently in error, the volume of an impossible frustum of a pyramid to arrive at the term 81−144 in his calculations, which today would simplify to ⁠−63=3i7⁠. Negative quantities were not conceived of in Hellenistic mathematics and Hero merely replaced the negative value by its positive ⁠144−81=37⁠.

The impetus to study complex numbers as a topic in itself first arose in the 16th century when algebraic solutions for the roots of cubic and quartic polynomials were discovered by Italian mathematicians (Niccolò Fontana Tartaglia and Gerolamo Cardano). It was soon realized (but proved much later) that these formulas, even if one were interested only in real solutions, sometimes required the manipulation of square roots of negative numbers. In fact, it was proved later that the use of complex numbers is unavoidable when all three roots are real and distinct. However, the general formula can still be used in this case, with some care to deal with the ambiguity resulting from the existence of three cubic roots for nonzero complex numbers. Rafael Bombelli was the first to address explicitly these seemingly paradoxical solutions of cubic equations and developed the rules for complex arithmetic, trying to resolve these issues.

The term "imaginary" for these quantities was coined by René Descartes in 1637, who was at pains to stress their unreal nature:

... sometimes only imaginary, that is one can imagine as many as I said in each equation, but sometimes there exists no quantity that matches that which we imagine.

[... quelquefois seulement imaginaires c'est-à-dire que l'on peut toujours en imaginer autant que j'ai dit en chaque équation, mais qu'il n'y a quelquefois aucune quantité qui corresponde à celle qu'on imagine.]

A further source of confusion was that the equation −12=−1−1=−1 seemed to be capriciously inconsistent with the algebraic identity ⁠ab=ab⁠, which is valid for non-negative real numbers a and b, and which was also used in complex number calculations with one of a, b positive and the other negative. The incorrect use of this identity in the case when both a and b are negative, and the related identity ⁠1a=1a⁠, even bedeviled Leonhard Euler. This difficulty eventually led to the convention of using the special symbol i in place of −1 to guard against this mistake. Even so, Euler considered it natural to introduce students to complex numbers much earlier than we do today. In his elementary algebra text book, Elements of Algebra, he introduces these numbers almost at once and then uses them in a natural way throughout.

In the 18th century complex numbers gained wider use, as it was noticed that formal manipulation of complex expressions could be used to simplify calculations involving trigonometric functions. For instance, in 1730 Abraham de Moivre noted that the identities relating trigonometric functions of an integer multiple of an angle to powers of trigonometric functions of that angle could be re-expressed by the following de Moivre's formula:

(cos⁡θ+isin⁡θ)n=cos⁡nθ+isin⁡nθ.

In 1748, Euler went further and obtained Euler's formula of complex analysis:

eiθ=cos⁡θ+isin⁡θ

by formally manipulating complex power series and observed that this formula could be used to reduce any trigonometric identity to much simpler exponential identities.

The idea of a complex number as a point in the complex plane was first described by Danish–Norwegian mathematician Caspar Wessel in 1799, although it had been anticipated as early as 1685 in Wallis's A Treatise of Algebra.

Wessel's memoir appeared in the Proceedings of the Copenhagen Academy but went largely unnoticed. In 1806 Jean-Robert Argand independently issued a pamphlet on complex numbers and provided a rigorous proof of the fundamental theorem of algebra. Carl Friedrich Gauss had earlier published an essentially topological proof of the theorem in 1797 but expressed his doubts at the time about "the true metaphysics of the square root of −1". It was not until 1831 that he overcame these doubts and published his treatise on complex numbers as points in the plane, largely establishing modern notation and terminology:

If one formerly contemplated this subject from a false point of view and therefore found a mysterious darkness, this is in large part attributable to clumsy terminology. Had one not called +1, −1, −1 positive, negative, or imaginary (or even impossible) units, but instead, say, direct, inverse, or lateral units, then there could scarcely have been talk of such darkness.

In the beginning of the 19th century, other mathematicians discovered independently the geometrical representation of the complex numbers: Buée, Mourey, Warren, Français and his brother, Bellavitis.

The English mathematician G.H. Hardy remarked that Gauss was the first mathematician to use complex numbers in "a really confident and scientific way" although mathematicians such as Norwegian Niels Henrik Abel and Carl Gustav Jacob Jacobi were necessarily using them routinely before Gauss published his 1831 treatise.

Augustin-Louis Cauchy and Bernhard Riemann together brought the fundamental ideas of complex analysis to a high state of completion, commencing around 1825 in Cauchy's case.

The common terms used in the theory are chiefly due to the founders. Argand called cos φ + i sin φ the direction factor, and r=a2+b2 the modulus; Cauchy (1821) called cos φ + i sin φ the reduced form (l'expression réduite) and apparently introduced the term argument; Gauss used i for −1, introduced the term complex number for a + bi, and called a2 + b2 the norm. The expression direction coefficient, often used for cos φ + i sin φ, is due to Hankel (1867), and absolute value, for modulus, is due to Weierstrass.

Later classical writers on the general theory include Richard Dedekind, Otto Hölder, Felix Klein, Henri Poincaré, Hermann Schwarz, Karl Weierstrass and many others. Important work (including a systematization) in complex multivariate calculus has been started at beginning of the 20th century. Important results have been achieved by Wilhelm Wirtinger in 1927.

## Abstract and algebraic definitions

While the above concrete definitions, including the addition and multiplication, accurately describe the complex numbers, there are other, equivalent approaches that reveal the abstract algebraic structure of the complex numbers more immediately.

One definition of the complex numbers is that they form a field denoted ⁠ℂ⁠ that contains the real field ⁠ℝ⁠, and is generated over ⁠ℝ⁠ by a distinguished element denoted ⁠i⁠ such that ⁠i2=−1⁠. Equivalently, ⁠ℂ⁠ is the splitting field of the polynomial ⁠x2+1⁠ over the real field ⁠ℝ⁠.

Existence of the field can be established by means of various models. One model is to regard the complex field as the set ⁠ℝ2⁠ equipped with addition and multiplication laws on ordered pairs ⁠(a,b)⁠ and ⁠(a′,b′)⁠:

(a,b)+(a′,b′)=(a+a′,b+b′),(a,b)(a′,b′)=(aa′−bb′,ab′+a′b).

In this model the element ⁠i⁠ corresponds to the element ⁠(0,1)⁠, and the real numbers are embedded as the set of pairs ⁠(a,0)⁠. Another is the standard construction of a splitting field using a quotient of the polynomial ring ⁠ℝ[X]⁠, given below.

Because the field ⁠ℂ⁠ has different models, they are not all literally the same mathematical object, but they are all isomorphic, up to a ring isomorphism preserving the real numbers. More precisely, given two models ⁠ℂ1=ℝ(i1)⁠ and ⁠ℂ2=ℝ(i2)⁠, there is a unique isomorphism ⁠ϕ:ℂ1→ℂ2⁠ which is linear over the real numbers and such that ⁠ϕ(i1)=i2⁠.

More generally, any non-trivial finite extension field of the reals is isomorphic to the complex field.  Moreover, the isomorphism is unique, provided it preserves the real subfield, up to the action of the Galois group (complex conjugation).

### Construction as a quotient ring

One approach to ⁠ℂ⁠ is via polynomials, i.e., expressions of the form

p(X)=anXn+⋯+a1X+a0,

where the coefficients a0, ..., an are real numbers. The set of all such polynomials is denoted by ℝ[X]. Since sums and products of polynomials are again polynomials, this set ℝ[X] forms the commutative polynomial ring (over the reals). To every such polynomial p, one may assign the complex number p(i)=anin+⋯+a1i+a0, i.e., the value obtained by setting X=i. This defines a function

ℝ[X]→ℂ

This function is surjective since every complex number can be obtained in such a way: the evaluation of a linear polynomial a+bX at X=i is a+bi. However, the evaluation of polynomial X2+1 at i is 0, since i2+1=0. This polynomial is irreducible, i.e., cannot be written as a product of two linear polynomials. Basic facts of abstract algebra then imply that the kernel of the above map is an ideal generated by this polynomial, and that the quotient by this ideal is a field, and that there is an isomorphism

ℝ[X]/(X2+1)→≅ℂ

between the quotient ring and ⁠ℂ⁠. Some authors take this as the definition of ⁠ℂ⁠. This definition expresses ⁠ℂ⁠ as a quadratic algebra.

The field ⁠ℂ⁠ is algebraically closed by the fundamental theorem of algebra, and is therefore the algebraic closure of ⁠ℝ⁠.

### Matrix representation of complex numbers

Complex numbers a + bi can also be represented by 2 × 2 matrices that have the form

(a−bba).

Here the entries a and b are real numbers. As the sum and product of two such matrices is again of this form, these matrices form a subring of the ring of 2 × 2 matrices.

A simple computation shows that the map

a+ib↦(a−bba)

is a ring isomorphism from the field of complex numbers to the ring of these matrices, proving that these matrices form a field. This isomorphism associates the square of the absolute value of a complex number with the determinant of the corresponding matrix, and the conjugate of a complex number with the transpose of the matrix.

The polar form representation of complex numbers explicitly gives these matrices as scaled rotation matrices.

r(cos⁡θ+isin⁡θ)↦(rcos⁡θ−rsin⁡θrsin⁡θrcos⁡θ)

In particular, the case of r = 1, which is |a+ib|=a2+b2=1, gives (unscaled) rotation matrices.

## Complex analysis

The study of functions of a complex variable is known as complex analysis and has enormous practical use in applied mathematics as well as in other branches of mathematics. Often, the most natural proofs for statements in real analysis or even number theory employ techniques from complex analysis (see prime number theorem for an example).

Unlike real functions, which are commonly represented as two-dimensional graphs, complex functions have four-dimensional graphs and may usefully be illustrated by color-coding a three-dimensional graph to suggest four dimensions, or by animating the complex function's dynamic transformation of the complex plane.

### Convergence

The notions of convergent series and continuous functions in (real) analysis have natural analogs in complex analysis. A sequence  of complex numbers is said to converge if and only if its real and imaginary parts do. This is equivalent to the (ε, δ)-definition of limits, where the absolute value of real numbers is replaced by the one of complex numbers. From a more abstract point of view, ⁠ℂ⁠, endowed with the metric

d⁡(z1,z2)=|z1−z2|

is a complete metric space, which notably includes the triangle inequality

|z1+z2|≤|z1|+|z2|

for any two complex numbers z1 and z2.

### Complex exponential

Like in real analysis, this notion of convergence is used to construct a number of elementary functions: the exponential function exp z, also written ez, is defined as the infinite series, which can be shown to converge for any z:

exp⁡z:=1+z+z22⋅1+z33⋅2⋅1+⋯=∑n=0∞znn!.

For example, exp⁡(1) is Euler's number e≈2.718. Euler's formula states:

exp⁡(iφ)=cos⁡φ+isin⁡φ

for any real number φ. This formula is a quick consequence of general basic facts about convergent power series and the definitions of the involved functions as power series. As a special case, this includes Euler's identity

exp⁡(iπ)=−1.

### Complex logarithm

For any positive real number t, there is a unique real number x such that exp⁡(x)=t. This leads to the definition of the natural logarithm as the inverse

ln⁡:ℝ+→ℝ;x↦ln⁡x of the exponential function. The situation is different for complex numbers, since

exp⁡(z+2πi)=exp⁡zexp⁡(2πi)=exp⁡z

by the functional equation and Euler's identity.

For example, eiπ = e3iπ = −1 , so both iπ and 3iπ are possible values for the complex logarithm of −1.

In general, given any non-zero complex number w, any number z solving the equation

exp⁡z=w

is a complex logarithm of w, denoted log⁡w. It can be shown that these numbers satisfy

z=log⁡w=ln⁡|w|+iarg⁡w,

where arg is the argument defined above, and ln the (real) natural logarithm. As arg is a multivalued function, unique only up to a multiple of 2π, log is also multivalued. The principal value of log is often taken by restricting the imaginary part to the interval (−π, π]. This leads to the complex logarithm being a bijective function taking values in the strip ℝ++i(−π,π] (that is denoted S0 in the above illustration)

ln⁡:ℂ×→ℝ++i(−π,π].

If z∈ℂ∖(−ℝ≥0) is not a non-positive real number (a positive or a non-real number), the resulting principal value of the complex logarithm is obtained with −π < φ < π. It is an analytic function outside the negative real numbers, but it cannot be prolongated to a function that is continuous at any negative real number z∈−ℝ+, where the principal value is ln z = ln(−z) + iπ.

Complex exponentiation zω is defined as

zω=exp⁡(ωln⁡z),

and is multi-valued, except when ω is an integer. For ω = 1 / n, for some natural number n, this recovers the non-uniqueness of nth roots mentioned above. If z > 0 is real (and ω an arbitrary complex number), one has a preferred choice of ln⁡x, the real logarithm, which can be used to define a preferred exponential function.

Complex numbers, unlike real numbers, do not in general satisfy the unmodified power and logarithm identities, particularly when naïvely treated as single-valued functions; see failure of power and logarithm identities. For example, they do not satisfy

abc=(ab)c.

Both sides of the equation are multivalued by the definition of complex exponentiation given here, and the values on the left are a subset of those on the right.

### Complex sine and cosine

The series defining the real trigonometric functions sin and cos, as well as the hyperbolic functions sinh and cosh, also carry over to complex arguments without change. For the other trigonometric and hyperbolic functions, such as tan, things are slightly more complicated, as the defining series do not converge for all complex values. Therefore, one must define them either in terms of sine, cosine and exponential, or, equivalently, by using the method of analytic continuation.

The value of a trigonometric or hyperbolic function of a complex number can be expressed in terms of those functions evaluated on real numbers, via angle-addition formulas.  For z = x + iy,

sin⁡z=sin⁡xcosh⁡y+icos⁡xsinh⁡y

cos⁡z=cos⁡xcosh⁡y−isin⁡xsinh⁡y

tan⁡z=tan⁡x+itanh⁡y1−itan⁡xtanh⁡y

cot⁡z=−1+icot⁡xcoth⁡ycot⁡x−icoth⁡y

sinh⁡z=sinh⁡xcos⁡y+icosh⁡xsin⁡y

cosh⁡z=cosh⁡xcos⁡y+isinh⁡xsin⁡y

tanh⁡z=tanh⁡x+itan⁡y1+itanh⁡xtan⁡y

coth⁡z=1−icoth⁡xcot⁡ycoth⁡x−icot⁡y

Where these expressions are not well defined, because a trigonometric or hyperbolic function evaluates to infinity or there is division by zero, they are nonetheless correct as limits.

### Holomorphic functions

A function f:ℂ→ℂ is holomorphic or complex differentiable at a point z0 if the limit

limz→z0f(z)−f(z0)z−z0

exists (in which case it is denoted by f′(z0)). This mimics the definition for real differentiable functions, except that all quantities are complex numbers. Loosely speaking, the freedom of approaching z0 in different directions imposes a much stronger condition than being (real) differentiable. For example, the function

f(z)=z‾

is differentiable as a function ℝ2→ℝ2, but is not complex differentiable.

A real differentiable function is complex differentiable if and only if it satisfies the Cauchy–Riemann equations, which are sometimes abbreviated as

∂f∂z‾=0.

Complex analysis shows some features not apparent in real analysis. For example, the identity theorem asserts that two holomorphic functions f and g agree if they agree on an arbitrarily small open subset of ⁠ℂ⁠. Meromorphic functions, functions that can locally be written as f(z)/(z − z0)n with a holomorphic function f, still share some of the features of holomorphic functions. Other functions have essential singularities, such as sin(1/z) at z = 0.

## Applications

Complex numbers have applications in many scientific areas, including signal processing, control theory, electromagnetism, fluid dynamics, quantum mechanics, cartography, and vibration analysis. Some of these applications are described below.

Complex conjugation is also employed in inversive geometry, a branch of geometry studying reflections more general than ones about a line. In the network analysis of electrical circuits, the complex conjugate is used in finding the equivalent impedance when the maximum power transfer theorem is looked for.

#### Shapes

Three non-collinear points u,v,w in the plane determine the shape of the triangle {u,v,w}. Locating the points in the complex plane, this shape of a triangle may be expressed by complex arithmetic as

S(u,v,w)=u−wu−v.

The shape S of a triangle will remain the same, when the complex plane is transformed by translation or dilation (by an affine transformation), corresponding to the intuitive notion of shape, and describing similarity. Thus each triangle {u,v,w} is in a similarity class of triangles with the same shape.

#### Fractal geometry

The Mandelbrot set is a popular example of a fractal formed on the complex plane. It is defined by plotting every location c where iterating the sequence fc(z)=z2+c does not diverge when iterated infinitely. Similarly, Julia sets have the same rules, except where c remains constant.

#### Triangles

Every triangle has a unique Steiner inellipse – an ellipse inside the triangle and tangent to the midpoints of the three sides of the triangle. The foci of a triangle's Steiner inellipse can be found as follows, according to Marden's theorem: Denote the triangle's vertices in the complex plane as a = xA + yAi, b = xB + yBi, and c = xC + yCi. Write the cubic equation (x−a)(x−b)(x−c)=0, take its derivative, and equate the (quadratic) derivative to zero. Marden's theorem says that the solutions of this equation are the complex numbers denoting the locations of the two foci of the Steiner inellipse.

### Algebraic number theory

As mentioned above, any nonconstant polynomial equation (in complex coefficients) has a solution in ⁠ℂ⁠. A fortiori, the same is true if the equation has rational coefficients. The roots of such equations are called algebraic numbers – they are a principal object of study in algebraic number theory. Compared to ℚ‾, the algebraic closure of ⁠ℚ⁠, which also contains all algebraic numbers, ⁠ℂ⁠ has the advantage of being easily understandable in geometric terms. In this way, algebraic methods can be used to study geometric questions and vice versa. With algebraic methods, more specifically applying the machinery of field theory to the number field containing roots of unity, it can be shown that it is not possible to construct a regular nonagon using only compass and straightedge – a purely geometric problem.

Another example is the Gaussian integers; that is, numbers of the form x + iy, where x and y are integers, which can be used to classify sums of squares.

### Analytic number theory

Analytic number theory studies numbers, often integers or rationals, by taking advantage of the fact that they can be regarded as complex numbers, in which analytic methods can be used. This is done by encoding number-theoretic information in complex-valued functions. For example, the Riemann zeta function ζ(s) is related to the distribution of prime numbers.

### Improper integrals

In applied fields, complex numbers are often used to compute certain real-valued improper integrals, by means of complex-valued functions. Several methods exist to do this; see methods of contour integration.

### Dynamic equations

In differential equations, it is common to first find all complex roots r of the characteristic equation of a linear differential equation or equation system and then attempt to solve the system in terms of base functions of the form f(t) = ert. Likewise, in difference equations, the complex roots r of the characteristic equation of the difference equation system are used, to attempt to solve the system in terms of base functions of the form f(t) = rt.

### Linear algebra

Since ⁠ℂ⁠ is algebraically closed, any non-empty complex square matrix has at least one (complex) eigenvalue. By comparison, real matrices do not always have real eigenvalues, for example rotation matrices (for rotations of the plane for angles other than 0° or 180°) leave no direction fixed, and therefore do not have any real eigenvalue. The existence of (complex) eigenvalues, and the ensuing existence of eigendecomposition is a useful tool for computing matrix powers and matrix exponentials.

Complex numbers often generalize concepts originally conceived in the real numbers. For example, the conjugate transpose generalizes the transpose, hermitian matrices generalize symmetric matrices, and unitary matrices generalize orthogonal matrices.

#### Control theory

In control theory, systems are often transformed from the time domain to the complex frequency domain using the Laplace transform. The system's zeros and poles are then analyzed in the complex plane. The root locus, Nyquist plot, and Nichols plot techniques all make use of the complex plane.

In the root locus method, it is important whether zeros and poles are in the left or right half planes, that is, have real part greater than or less than zero. If a linear, time-invariant (LTI) system has poles that are

in the right half plane, it will be unstable,

all in the left half plane, it will be stable,

on the imaginary axis, it will have marginal stability.

If a system has zeros in the right half plane, it is a nonminimum phase system.

#### Signal analysis

Complex numbers are used in signal analysis and other fields for a convenient description for periodically varying signals. For given real functions representing actual physical quantities, often in terms of sines and cosines, corresponding complex functions are considered of which the real parts are the original quantities. For a sine wave of a given frequency, the absolute value |z| of the corresponding z is the amplitude and the argument arg z is the phase.

If Fourier analysis is employed to write a given real-valued signal as a sum of periodic functions, these periodic functions are often written as complex-valued functions of the form

x(t)=Re⁡{X(t)}

and

X(t)=Aeiωt=aeiϕeiωt=aei(ωt+ϕ)

where ω represents the angular frequency and the complex number A encodes the phase and amplitude as explained above.

This use is also extended into digital signal processing and digital image processing, which use digital versions of Fourier analysis (and wavelet analysis) to transmit, compress, restore, and otherwise process digital audio signals, still images, and video signals.

#### Electromagnetism and electrical engineering

In electrical engineering, the Fourier transform is used to analyze varying electric currents and voltages. The treatment of resistors, capacitors, and inductors can then be unified by introducing imaginary, frequency-dependent resistances for the latter two and combining all three in a single complex number called the impedance. This approach is called phasor calculus.

In electrical engineering, the imaginary unit is denoted by j, to avoid confusion with I, which is generally in use to denote electric current, or, more particularly, i, which is generally in use to denote instantaneous electric current.

Because the voltage in an AC circuit is oscillating, it can be represented as

V(t)=V0ejωt=V0(cos⁡ωt+jsin⁡ωt),

To obtain the measurable quantity, the real part is taken:

v(t)=Re⁡(V)=Re⁡[V0ejωt]=V0cos⁡ωt.

The complex-valued signal V(t) is called the analytic representation of the real-valued, measurable signal v(t).

#### Fluid dynamics

In fluid dynamics, complex functions are used to describe potential flow in two dimensions.

#### Quantum mechanics

The complex number field is intrinsic to the standard mathematical formulations of quantum mechanics, where complex Hilbert spaces provide the context for one such formulation that is convenient and perhaps most standard. The original foundation formulas of quantum mechanics – the Schrödinger equation and Heisenberg's matrix mechanics – make use of complex numbers.

#### Relativity

In special relativity and general relativity, some formulas for the metric on spacetime become simpler if one takes the time component of the spacetime continuum to be imaginary. (This approach is no longer standard in classical relativity, but is used in an essential way in quantum field theory.) Complex numbers are essential to spinors, which are a generalization of the tensors used in relativity.

### Algebraic characterization

The field ⁠ℂ⁠ has the following three properties:

First, it has characteristic 0. This means that 1 + 1 + ⋯ + 1 ≠ 0 for any number of summands (all of which equal one).

Second, its transcendence degree over ⁠ℚ⁠, the prime field of ⁠ℂ⁠, is the cardinality of the continuum.

Third, it is algebraically closed (see above).

It can be shown that any field having these properties is isomorphic (as a field) to ⁠ℂ⁠. For example, the algebraic closure of the field ⁠ℚp⁠ of the p-adic number also satisfies these three properties, so these two fields are isomorphic (as fields, but not as topological fields). Also, ⁠ℂ⁠ is isomorphic to the field of complex Puiseux series. However, specifying an isomorphism requires the axiom of choice. Another consequence of this algebraic characterization is that ⁠ℂ⁠ contains many proper subfields that are isomorphic to ⁠ℂ⁠.

### Characterization as a topological field

The preceding characterization of ⁠ℂ⁠ describes only the algebraic aspects of ⁠ℂ⁠. That is to say, the properties of nearness and continuity, which matter in areas such as analysis and topology, are not dealt with. The following description of ⁠ℂ⁠ as a topological field (that is, a field that is equipped with a topology, which allows the notion of convergence) does take into account the topological properties. ⁠ℂ⁠ contains a subset P (namely the set of positive real numbers) of nonzero elements satisfying the following three conditions:

P is closed under addition, multiplication and taking inverses.

If x and y are distinct elements of P, then either x − y or y − x is in P.

If S is any nonempty subset of P, then S + P = x + P for some x in ⁠ℂ⁠.

Moreover, ⁠ℂ⁠ has a nontrivial involutive automorphism x ↦ x* (namely complex conjugation), such that x x* is in P for any nonzero x in ⁠ℂ⁠.

Any field F with these properties can be endowed with a topology by taking the sets B(x, p) = { y | p − (y − x)(y − x)* ∈ P } as a base, where x ranges over the field and p ranges over P. With this topology F is isomorphic as a topological field to ⁠ℂ⁠.

The only connected locally compact topological fields are ⁠ℝ⁠ and ⁠ℂ⁠. This gives another characterization of ⁠ℂ⁠ as a topological field, because ⁠ℂ⁠ can be distinguished from ⁠ℝ⁠ because the nonzero complex numbers are connected, while the nonzero real numbers are not.

### Other number systems

The process of extending the field ⁠ℝ⁠ of reals to ⁠ℂ⁠ is an instance of the Cayley–Dickson construction. Applying this construction iteratively to ⁠ℂ⁠ then yields the quaternions, the octonions, the sedenions, and the trigintaduonions. This construction turns out to diminish the structural properties of the involved number systems.

Unlike the reals, ⁠ℂ⁠ is not an ordered field, that is to say, it is not possible to define a relation z1 < z2 that is compatible with the addition and multiplication. In fact, in any ordered field, the square of any element is necessarily positive, so i2 = −1 precludes the existence of an ordering on ⁠ℂ⁠. Passing from ⁠ℂ⁠ to the quaternions ⁠ℍ⁠ loses commutativity, while the octonions (additionally to not being commutative) fail to be associative. The reals, complex numbers, quaternions and octonions are all normed division algebras over ⁠ℝ⁠. By Hurwitz's theorem they are the only ones; the sedenions, the next step in the Cayley–Dickson construction, fail to have this structure.

The Cayley–Dickson construction is closely related to the regular representation of ⁠ℂ⁠, thought of as an ⁠ℝ⁠-algebra (an ⁠ℝ⁠-vector space with a multiplication), with respect to the basis (1, i). This means the following: the ⁠ℝ⁠-linear map

ℂ→ℂz↦wz

for some fixed complex number w can be represented by a 2 × 2 matrix (once a basis has been chosen). With respect to the basis (1, i), this matrix is

(Re⁡(w)−Im⁡(w)Im⁡(w)Re⁡(w)),

that is, the one mentioned in the section on matrix representation of complex numbers above. While this is a linear representation of ⁠ℂ⁠ in the 2 × 2 real matrices, it is not the only one. Any matrix

J=(pqr−p),p2+qr+1=0

has the property that its square is the negative of the identity matrix: J2 = −I. Then

{z=aI+bJ:a,b∈ℝ}

is also isomorphic to the field ⁠ℂ⁠, and gives an alternative complex structure on ℝ2. This is generalized by the notion of a linear complex structure.

Hypercomplex numbers also generalize ⁠ℝ⁠, ⁠ℂ⁠, ⁠ℍ⁠, and ⁠𝕆⁠. For example, this notion contains the split-complex numbers, which are elements of the ring ℝ[x]/(x2−1) (as opposed to ℝ[x]/(x2+1) for complex numbers). In this ring, the equation a2 = 1 has four solutions.

The field ⁠ℝ⁠ is the completion of ⁠ℚ⁠, the field of rational numbers, with respect to the usual absolute value metric. Other choices of metrics on ⁠ℚ⁠ lead to the fields ⁠ℚp⁠ of p-adic numbers (for any prime number p), which are thereby analogous to ⁠ℝ⁠. There are no other nontrivial ways of completing ⁠ℚ⁠ than ⁠ℝ⁠ and ⁠ℚp⁠, by Ostrowski's theorem. The algebraic closures ℚp‾ of ⁠ℚp⁠ still carry a norm, but (unlike ⁠ℂ⁠) are not complete with respect to it. The completion ⁠ℂp⁠ of ℚp‾ turns out to be algebraically closed. By analogy, ⁠ℂp⁠ is called the p-adic complex numbers.

The fields ⁠ℝ⁠, ⁠ℚp⁠, and their finite field extensions, including ⁠ℂ⁠, are local fields.
