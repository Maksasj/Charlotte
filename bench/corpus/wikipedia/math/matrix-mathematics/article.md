# Matrix (mathematics)

In mathematics, a matrix (pl.: matrices) is a rectangular array of numbers or other mathematical objects with elements or entries arranged in rows and columns, usually satisfying certain properties of addition and multiplication.

For example,

[19−13205−6]

denotes a matrix with two rows and three columns. This is often referred to as a "two-by-three matrix", a 2 × 3 matrix, or a matrix of dimension 2 × 3.

In linear algebra, matrices are used as linear maps. In geometry, matrices are used for geometric transformations (for example rotations) and coordinate changes. In numerical analysis, many computational problems are solved by reducing them to a matrix computation, and this often involves computing with matrices of huge dimensions. Matrices are used in most areas of mathematics and scientific fields, either directly, or through their use in geometry and numerical analysis.

Square matrices, matrices with the same number of rows and columns, play a major role in matrix theory. The determinant of a square matrix is a number associated with the matrix, which is fundamental for the study of a square matrix. For example, a square matrix is invertible if and only if it has a nonzero determinant. Also, the eigenvalues of a square matrix are the roots of its characteristic polynomial, det⁡(λI−A).

Matrix theory is the branch of mathematics that focuses on the study of matrices. It was initially a sub-branch of linear algebra, but soon grew to include subjects related to graph theory, algebra, combinatorics and statistics.

## Definition

A matrix is a rectangular array of numbers (or other mathematical objects), called the "entries" of the matrix. Matrices are subject to standard operations such as addition and multiplication. Most commonly, a matrix over a field F is a rectangular array of elements of ⁠F⁠. A real matrix and a complex matrix are matrices whose entries are respectively real numbers or complex numbers. More general types of entries are discussed below. For instance, this is a real matrix:

𝐀=[−1.30.620.45.59.7−6.2].

The numbers (or other objects) in the matrix are called its entries or its elements. The horizontal and vertical lines of entries in a matrix are respectively called rows and columns.

### Size

The size of a matrix is defined by the number of rows and columns it contains. There is no limit to the number of rows and columns that a matrix (in the usual sense) can have as long as they are positive integers. A matrix with m rows and n columns is called an m × n matrix, or m-by-n matrix, where m and n are called its dimensions. For example, the matrix 𝐀 above is a 3 × 2 matrix.

Matrices with a single row are called row matrices or row vectors, and those with a single column are called column matrices or column vectors. A matrix with the same number of rows and columns is called a square matrix. A matrix with an infinite number of rows or columns (or both) is called an  infinite matrix. In some contexts, such as computer algebra programs, it is useful to consider a matrix with no rows or no columns, called an  empty matrix.

## Notation

The specifics of symbolic matrix notation vary widely, with some prevailing trends.  Matrices are commonly written in square brackets or parentheses, so that an m × n matrix 𝐀 is represented as

𝐀=[a11a12⋯a1na21a22⋯a2n⋮⋮⋱⋮am1am2⋯amn]=(a11a12⋯a1na21a22⋯a2n⋮⋮⋱⋮am1am2⋯amn).

This may be abbreviated by writing only a single generic term, possibly along with indices, as in

𝐀=(aij),[aij],or(aij)1≤i≤m,1≤j≤n

or 𝐀=(ai,j)1≤i,j≤n in the case that ⁠n=m⁠.

Matrices are usually symbolized using upper-case letters (such as 𝐀 in the examples above), while the corresponding lower-case letters, with two subscript indices (e.g., ⁠a11⁠, or ⁠a1,1⁠), represent the entries. In addition to using upper-case letters to symbolize matrices, many authors use a special typographical style, commonly boldface roman (non-italic), to further distinguish matrices from other mathematical objects. An alternative notation involves the use of a double-underline with the variable name, with or without boldface style, as in ⁠A__⁠.

The entry in the ith row and jth column of a matrix A is sometimes referred to as the i,j or (i,j) entry of the matrix, and commonly denoted by ai,j or ⁠aij⁠.  Alternative notations for that entry are 𝐀[i,j] and ⁠𝐀i,j⁠. For example, the (1,3) entry of the following matrix 𝐀 is 5 (also denoted ⁠a13⁠, ⁠a1,3⁠, 𝐀[1,3] or ⁠𝐀1,3⁠):

𝐀=[4−750−20118191−312]

Sometimes, the entries of a matrix can be defined by a formula such as ⁠ai,j=f(i,j)⁠. For example, each of the entries of the following matrix 𝐀 is determined by the formula ⁠aij=i−j⁠.

𝐀=[0−1−2−310−1−2210−1]

In this case, the matrix itself is sometimes defined by that formula, within square brackets or double parentheses. For example, the matrix above is defined as 𝐀=[i−j] or ⁠𝐀=((i−j))⁠. If matrix size is m × n, the above-mentioned formula f(i,j) is valid for any i=1,…,m and any ⁠j=1,…,n⁠. This can be specified separately or indicated using m × n as a subscript. For instance, the matrix 𝐀 above is 3 × 4, and can be defined as 𝐀=[i−j](i=1,2,3;j=1,…,4) or ⁠𝐀=[i−j]3×4⁠.

Some programming languages utilize doubly subscripted arrays (or arrays of arrays) to represent an {m-by-n matrix. Some programming languages start the numbering of array indexes at zero, in which case the entries of an m × n matrix are indexed by 0≤i≤m−1 and ⁠0≤j≤n−1⁠. This article follows the more common convention in mathematical writing where enumeration starts from 1.

The set of all m-by-n real matrices is often denoted ⁠ℳ(m,n)⁠, or ⁠ℳm×n(ℝ)⁠. The set of all m × n matrices over another field, or over a ring R, is similarly denoted ⁠ℳ(m,n,R)⁠, or ⁠ℳm×n(R)⁠. If m = n, such as in the case of square matrices, one does not repeat the dimension: ⁠ℳ(n,R)⁠, or ⁠ℳn(R)⁠. Often, ⁠M⁠, or ⁠Mat⁠, is used in place of ⁠ℳ⁠.

## Basic operations

Several basic operations can be applied to matrices. Some, such as transposition and submatrix do not depend on the nature of the entries. Others, such as matrix addition, scalar multiplication, matrix multiplication, and row operations involve operations on matrix entries and therefore require that matrix entries are numbers or belong to a field or a ring.

In this section, it is supposed that matrix entries belong to a fixed ring, which is typically a field of numbers.

### Addition

Matrix addition and subtraction require matrices of a consistent size, and are calculated entrywise. The sum A + B and the difference A − B of two m × n matrices are:

(𝐀+𝐁)i,j=𝐀i,j+𝐁i,j,1≤i≤m,1≤j≤n.(𝐀−𝐁)i,j=𝐀i,j−𝐁i,j,1≤i≤m,1≤j≤n.

For example,

[131100]+[005750]=[1+03+01+51+70+50+0]=[136850]

Familiar properties of numbers extend to these operations on matrices: for example, addition is commutative, that is, the matrix sum does not depend on the order of the summands: A + B = B + A.

### Scalar multiplication

The product cA of a number c (also called a scalar in this context) and a matrix A is computed by multiplying each entry of A by c:

(c𝐀)i,j=c⋅𝐀i,j

This operation is called scalar multiplication, but its result is not named "scalar product" to avoid confusion, since "scalar product" is often used as a synonym for "inner product". For example:

2⋅[18−34−25]=[2⋅12⋅82⋅−32⋅42⋅−22⋅5]=[216−68−410]

Matrix subtraction is consistent with composition of matrix addition with scalar multiplication by –1:

𝐀−𝐁=𝐀+(−1)⋅𝐁

### Transpose

The transpose of an m × n matrix A is the n × m matrix AT (also denoted Atr or tA) formed by turning rows into columns and vice versa:

(𝐀T)i,j=𝐀j,i.

For example:

[1230−67]T=[102−637]

The transpose is compatible with addition and scalar multiplication, as expressed by (cA)T = c(AT) and (A + B)T = AT + BT. Finally, (AT)T = A.

### Matrix multiplication

Multiplication of two matrices corresponds to the composition of linear transformations represented by each matrix. It is defined if and only if the number of columns of the left matrix is the same as the number of rows of the right matrix. If A is an m × n matrix and B is an n × p matrix, then their matrix product AB is the m × p matrix whose entries are given by the dot product of the corresponding row of A and the corresponding column of B:

[𝐀𝐁]i,j=ai,1b1,j+ai,2b2,j+⋯+ai,nbn,j=∑r=1nai,rbr,j,

where 1 ≤ i ≤ m and 1 ≤ j ≤ p. For example, the underlined entry 2340 in the product is calculated as (2 × 1000) + (3 × 100) + (4 × 10) = 2340:

[2_3_4_100][01000_1100_010_]=[32340_01000].

Matrix multiplication satisfies the rules (AB)C = A(BC) (associativity), and (A + B)C = AC + BC as well as C(A + B) = CA + CB (left and right distributivity), whenever the size of the matrices is such that the various products are defined. The product AB may be defined without BA being defined, namely if A and B are m × n and n × k matrices, respectively, and m ≠ k. Even if both products are defined, they generally need not be equal, that is:

𝐀𝐁≠𝐁𝐀.

In other words, matrix multiplication is not commutative, in marked contrast to (rational, real, or complex) numbers, whose product is independent of the order of the factors. An example of two matrices not commuting with each other is:

[1234][0100]=[0103],

whereas

[0100][1234]=[3400].

Besides the ordinary matrix multiplication just described, other less frequently used operations on matrices that can be considered forms of multiplication also exist, such as the Hadamard product and the Kronecker product. They arise in solving matrix equations such as the Sylvester equation.

### Row operations

There are three types of row operations:

row addition, that is, adding a row to another.

row multiplication, that is, multiplying all entries of a row by a non-zero constant;

row switching, that is, interchanging two rows of a matrix;

These operations are used in several ways, including solving linear equations and finding matrix inverses with Gauss elimination and Gauss–Jordan elimination, respectively.

### Submatrix

A submatrix of a matrix is a matrix obtained by deleting any collection of rows or columns or both. For example, from the following 3 × 4 matrix, we can construct a 2 × 3 submatrix by removing row 3 and column 2:

𝐀=[123456789101112]→[134578].

The minors and cofactors of a matrix are found by computing the determinant of certain submatrices.

A principal submatrix is a square submatrix obtained by removing certain rows and columns. The definition varies from author to author. According to some authors, a principal submatrix is a submatrix in which the set of row indices that remain is the same as the set of column indices that remain. Other authors define a principal submatrix as one in which the first k rows and columns, for some number k, are the ones that remain; this type of submatrix has also been called a leading principal submatrix.

## Linear equations

Matrices can be used to compactly write and work with multiple linear equations, that is, systems of linear equations. For example, if A is an m × n matrix, x designates a column vector (that is, n × 1 matrix) of n variables x1, x2, ..., xn, and b is an m × 1 column vector, then the matrix equation

𝐀𝐱=𝐛

is equivalent to the system of linear equations

a1,1x1+a1,2x2+⋯+a1,nxn=b1  ⋮am,1x1+am,2x2+⋯+am,nxn=bm

Using matrices, this can be solved more compactly than would be possible by writing out all the equations separately. If n = m and the equations are independent, then this can be done by writing

𝐱=𝐀−1𝐛

where A−1 is the inverse matrix of A. If A has no inverse, solutions—if any—can be found using its generalized inverse.

## Linear transformations

Matrices and matrix multiplication reveal their essential features when related to linear transformations, also known as linear maps. A real m-by-n matrix A gives rise to a linear transformation ℝn→ℝm mapping each vector x in ⁠ℝn⁠ to the (matrix) product Ax, which is a vector in ⁠ℝm.⁠ Conversely, each linear transformation f:ℝn→ℝm arises from a unique m-by-n matrix A: explicitly, the (i, j)-entry of A is the ith coordinate of f (ej), where ej = (0, ..., 0, 1, 0, ..., 0) is the unit vector with 1 in the jth position and 0 elsewhere. The matrix A is said to represent the linear map f, and A is called the transformation matrix of f.

For example, the 2 × 2 matrix

𝐀=[acbd]

can be viewed as the transform of the unit square into a parallelogram with vertices at (0, 0), (a, b), (a + c, b + d), and (c, d). The parallelogram pictured at the right is obtained by multiplying A with each of the column vectors ⁠[00]⁠, ⁠[10]⁠, ⁠[11]⁠, and ⁠[01]⁠ in turn. These vectors define the vertices of the unit square. The following table shows several 2 × 2 real matrices with the associated linear maps of ⁠ℝ2⁠. The blue original is mapped to the green grid and shapes. The origin (0, 0) is marked with a black point.

Under the 1-to-1 correspondence between matrices and linear maps, matrix multiplication corresponds to composition of maps: if a k-by-m matrix B represents another linear map ⁠g:ℝm→ℝk⁠, then the composition g ∘ f is represented by BA since

(g∘f)(𝐱)=g(f(𝐱))=g(𝐀𝐱)=𝐁(𝐀𝐱)=(𝐁𝐀)𝐱.

The last equality follows from the above-mentioned associativity of matrix multiplication.

The rank of a matrix A is the maximum number of linearly independent row vectors of the matrix, which is the same as the maximum number of linearly independent column vectors. Equivalently it is the dimension of the image of the linear map represented by A. The rank–nullity theorem states that the dimension of the kernel of a matrix plus the rank equals the number of columns of the matrix.

## Square matrix

A square matrix is a matrix with the same number of rows and columns. An n-by-n matrix is known as a square matrix of order n. Any two square matrices of the same order can be added and multiplied.

The entries aii form the main diagonal of a square matrix. They lie on the imaginary line running from the top left corner to the bottom right corner of the matrix.

Square matrices of a given dimension form a noncommutative ring, which is one of the most common examples of a noncommutative ring.

#### Diagonal and triangular matrix

If all entries of A below the main diagonal are zero, A is called an upper triangular matrix. Similarly, if all entries of A above the main diagonal are zero, A is called a lower triangular matrix. If all entries outside the main diagonal are zero, A is called a diagonal matrix.

#### Identity matrix

The identity matrix In of size n is the n-by-n matrix in which all the elements on the main diagonal are equal to 1 and all other elements are equal to 0, for example,

𝐈1=[1],𝐈2=[1001],⋮𝐈n=[10⋯001⋯0⋮⋮⋱⋮00⋯1]

It is a square matrix of order n, and also a special kind of diagonal matrix. It is called an identity matrix because multiplication with it leaves a matrix unchanged:

𝐀𝐈n=𝐈m𝐀=𝐀

for any m-by-n matrix A.

A scalar multiple of an identity matrix is called a scalar matrix.

#### Symmetric or skew-symmetric matrix

A square matrix A that is equal to its transpose, that is, A = AT, is a symmetric matrix. If instead, A is equal to the negative of its transpose, that is, A = −AT, then A is a skew-symmetric matrix. In complex matrices, symmetry is often replaced by the concept of Hermitian matrices, which satisfies A∗ = A, where the star or asterisk denotes the conjugate transpose of the matrix, that is, the transpose of the complex conjugate of A.

By the spectral theorem, real symmetric matrices and complex Hermitian matrices have an eigenbasis; that is, every vector is expressible as a linear combination of eigenvectors. In both cases, all eigenvalues are real. This theorem can be generalized to infinite-dimensional situations related to matrices with infinitely many rows and columns.

#### Invertible matrix and its inverse

A square matrix A is called invertible or non-singular if there exists a matrix B such that

𝐀𝐁=𝐁𝐀=𝐈n,

where In is the n × n identity matrix with 1 for each entry on the main diagonal and 0 elsewhere. If B exists, it is unique and is called the inverse matrix of A, denoted A−1.

There are many algorithms for testing whether a square matrix is invertible, and, if it is, computing its inverse. One of the oldest, which is still in common use is Gaussian elimination.

#### Definite matrix

A symmetric real matrix A is called positive-definite if the associated quadratic form

f(𝐱)=𝐱T𝐀𝐱

has a positive value for every nonzero vector x in ⁠ℝn⁠. If f(x) yields only negative values then A is negative-definite; if f does produce both negative and positive values then A is  indefinite. If the quadratic form f yields only non-negative values (positive or zero), the symmetric matrix is called positive-semidefinite (or if only non-positive values, then negative-semidefinite); hence the matrix is indefinite precisely when it is neither positive-semidefinite nor negative-semidefinite.

A symmetric matrix is positive-definite if and only if all its eigenvalues are positive, that is, the matrix is positive-semidefinite and it is invertible. The table at the right shows two possibilities for 2-by-2 matrices. The eigenvalues of a diagonal matrix are simply the entries along the diagonal, and so in these examples, the eigenvalues can be read directly from the matrices themselves. The first matrix has two eigenvalues that are both positive, while the second has one that is positive and another that is negative.

Allowing as input two different vectors instead yields the bilinear form associated to A:

B𝐀(𝐱,𝐲)=𝐱T𝐀𝐲.

In the case of complex matrices, the same terminology and results apply, with symmetric matrix, quadratic form, bilinear form, and transpose xT replaced respectively by Hermitian matrix, Hermitian form, sesquilinear form, and conjugate transpose xH.

#### Orthogonal matrix

An orthogonal matrix is a square matrix with real entries whose columns and rows are orthogonal unit vectors (that is, orthonormal vectors). Equivalently, a matrix A is orthogonal if its transpose is equal to its inverse:

𝐀T=𝐀−1,

which entails

𝐀T𝐀=𝐀𝐀T=𝐈n,

where In is the identity matrix of size n.

An orthogonal matrix A is necessarily invertible (with inverse A−1 = AT), unitary (A−1 = A*), and normal (A*A = AA*). The determinant of any orthogonal matrix is either +1 or −1. A special orthogonal matrix is an orthogonal matrix with determinant +1. As a linear transformation, every orthogonal matrix with determinant +1 is a pure rotation without reflection, i.e., the transformation preserves the orientation of the transformed structure, while every orthogonal matrix with determinant −1 reverses the orientation, i.e., is a composition of a pure reflection and a (possibly null) rotation. The identity matrices have determinant 1 and are pure rotations by an angle zero.

The complex analog of an orthogonal matrix is a unitary matrix.

#### Trace

The trace, tr(A) of a square matrix A is the sum of its diagonal entries. While matrix multiplication is not commutative as mentioned above, the trace of the product of two matrices is independent of the order of the factors:

tr⁡(𝐀𝐁)=tr⁡(𝐁𝐀).

This is immediate from the definition of matrix multiplication:

tr⁡(𝐀𝐁)=∑i=1m∑j=1naijbji=tr⁡(𝐁𝐀).

It follows that the trace of the product of more than two matrices is independent of cyclic permutations of the matrices; however, this does not in general apply for arbitrary permutations. For example, tr(ABC) ≠ tr(BAC), in general. Also, the trace of a matrix is equal to that of its transpose, that is,

tr⁡(𝐀)=tr⁡(𝐀T).

#### Determinant

The determinant of a square matrix A (denoted det(A) or |A|) is a number encoding certain properties of the matrix. A matrix is invertible if and only if its determinant is nonzero. Its absolute value equals the area (in ⁠ℝ2⁠) or volume (in ⁠ℝ3⁠) of the image of the unit square (or cube), while its sign corresponds to the orientation of the corresponding linear map: the determinant is positive if and only if the orientation is preserved.

The determinant of 2 × 2 matrices is given by

det⁡[abcd]=ad−bc.

The determinant of 3 × 3 matrices involves six terms (rule of Sarrus). The more lengthy Leibniz formula generalizes these two formulae to all dimensions.

The determinant of a product of square matrices equals the product of their determinants:

det⁡(𝐀𝐁)=det⁡(𝐀)⋅det⁡(𝐁),

or using alternate notation:

|𝐀𝐁|=|𝐀|⋅|𝐁|.

Adding a multiple of any row to another row, or a multiple of any column to another column, does not change the determinant. Interchanging two rows or two columns affects the determinant by multiplying it by −1. Using these operations, any matrix can be transformed to a lower (or upper) triangular matrix, and for such matrices, the determinant equals the product of the entries on the main diagonal; this provides a method to calculate the determinant of any matrix. Finally, the Laplace expansion expresses the determinant in terms of minors, that is, determinants of smaller matrices. This expansion can be used for a recursive definition of determinants (taking as starting case the determinant of a 1 × 1 matrix, which is its unique entry, or even the determinant of a 0 × 0 matrix, which is 1), that can be seen to be equivalent to the Leibniz formula. Determinants can be used to solve linear systems using Cramer's rule, where the division of the determinants of two related square matrices equates to the value of each of the system's variables.

#### Eigenvalues and eigenvectors

A number λ and a nonzero vector v satisfying

𝐀𝐯=λ𝐯

are called an eigenvalue and an eigenvector of A, respectively. The number λ is an eigenvalue of an n × n matrix A if and only if (A − λIn) is not invertible, which is equivalent to

det⁡(𝐀−λ𝐈)=0.

The polynomial pA in an indeterminate X given by evaluation of the determinant det(XIn − A) is called the characteristic polynomial of A. It is a monic polynomial of degree n. Therefore the polynomial equation pA(λ) = 0 has at most n different solutions, that is, eigenvalues of the matrix. They may be complex even if the entries of A are real. According to the Cayley–Hamilton theorem, pA(A) = 0, that is, the result of substituting the matrix itself into its characteristic polynomial yields the zero matrix.

## Computational aspects

Matrix calculations can be often performed with different techniques. Many problems can be solved by both direct algorithms and iterative approaches. For example, the eigenvectors of a square matrix can be obtained by finding a sequence of vectors xn converging to an eigenvector when n tends to infinity.

To choose the most appropriate algorithm for each specific problem, it is important to determine both the effectiveness and precision of all the available algorithms. The domain studying these matters is called numerical linear algebra. As with other numerical situations, two main aspects are the complexity of algorithms and their numerical stability.

Determining the complexity of an algorithm means finding upper bounds or estimates of how many elementary operations such as additions and multiplications of scalars are necessary to perform some algorithm, for example, multiplication of matrices. Calculating the matrix product of two n-by-n matrices using the definition given above needs n3 multiplications, since for any of the n2 entries of the product, n multiplications are necessary. The Strassen algorithm outperforms this "naive" algorithm; it needs only n2.807 multiplications.  Theoretically faster but impractical matrix multiplication algorithms have been developed, as have speedups to this problem using parallel algorithms or distributed computation systems such as MapReduce.

In many practical situations, additional information about the matrices involved is known. An important case concerns sparse matrices, that is, matrices whose entries are mostly zero. There are specifically adapted algorithms for, say, solving linear systems Ax = b for sparse matrices A, such as the conjugate gradient method.

An algorithm is, roughly speaking, numerically stable if little deviations in the input values do not lead to big deviations in the result. For example, one can calculate the inverse of a matrix by computing its adjugate matrix:

𝐀−1=adj⁡(𝐀)/det⁡(𝐀).

However, this may lead to significant rounding errors if the determinant of the matrix is very small. The norm of a matrix can be used to capture the conditioning of linear algebraic problems, such as computing a matrix's inverse.

## Decomposition

There are several methods to render matrices into a more easily accessible form. They are generally referred to as matrix decomposition or matrix factorization techniques. These techniques are of interest because they can make computations easier.

The LU decomposition factors matrices as a product of lower (L) and an upper triangular matrices (U). Once this decomposition is calculated, linear systems can be solved more efficiently by a simple technique called forward and back substitution. Likewise, inverses of triangular matrices are algorithmically easier to calculate. The Gaussian elimination is a similar algorithm; it transforms any matrix to row echelon form. Both methods proceed by multiplying the matrix by suitable elementary matrices, which correspond to permuting rows or columns and adding multiples of one row to another row. Singular value decomposition (SVD) expresses any matrix A as a product UDV∗, where U and V are unitary matrices and D is a diagonal matrix.

The eigendecomposition or diagonalization expresses A as a product VDV−1, where D is a diagonal matrix and V is a suitable invertible matrix. If A can be written in this form, it is called diagonalizable. More generally, and applicable to all matrices, the Jordan decomposition transforms a matrix into Jordan normal form, that is to say matrices whose only nonzero entries are the eigenvalues λ1 to λn of A, placed on the main diagonal and possibly entries equal to one directly above the main diagonal, as shown at the right. Given the eigendecomposition, the nth power of A (that is, n-fold iterated matrix multiplication) can be calculated via

𝐀n=(𝐕𝐃𝐕−1)n=𝐕𝐃𝐕−1𝐕𝐃𝐕−1…𝐕𝐃𝐕−1=𝐕𝐃n𝐕−1

and the power of a diagonal matrix can be calculated by taking the corresponding powers of the diagonal entries, which is much easier than doing the exponentiation for A instead. This can be used to compute the matrix exponential eA, a need frequently arising in solving linear differential equations, matrix logarithms and square roots of matrices. To avoid numerically ill-conditioned situations, further algorithms such as the Schur decomposition can be employed.

## Abstract algebraic aspects and generalizations

Matrices can be generalized in different ways. Abstract algebra uses matrices with entries in more general fields or even rings, while linear algebra codifies properties of matrices in the notion of linear maps. It is possible to consider matrices with infinitely many columns and rows. Another extension is tensors, which can be seen as higher-dimensional arrays of numbers, as opposed to vectors, which can often be realized as sequences of numbers, while matrices are rectangular or two-dimensional arrays of numbers. Matrices, subject to certain requirements tend to form groups known as matrix groups. Similarly under certain conditions matrices form rings known as matrix rings. Though the product of matrices is not in general commutative, certain matrices form fields sometimes called matrix fields. (However the term "matrix field" is ambiguous, also referring to certain forms of physical fields that continuously map points of some space to matrices.) In general, matrices over any ring and their multiplication can be represented as the arrows and composition of arrows in a category, the category of matrices over that ring. The objects of this category are natural numbers, representing the dimensions of the matrices.

### Matrices with entries in a field or ring

This article focuses on matrices whose entries are real or complex numbers. However, matrices can be considered with much more general types of entries than real or complex numbers. As a first step of generalization, any field, that is, a set where addition, subtraction, multiplication, and division operations are defined and well-behaved, may be used instead of ⁠ℝ⁠ or ⁠ℂ⁠, for example rational numbers or finite fields. For example, coding theory makes use of matrices over finite fields. Wherever eigenvalues are considered, as these are roots of a polynomial, they may exist only in a larger field than that of the entries of the matrix. For instance, they may be complex in the case of a matrix with real entries. The possibility to reinterpret the entries of a matrix as elements of a larger field (for example, to view a real matrix as a complex matrix whose entries happen to be all real) then allows considering each square matrix to possess a full set of eigenvalues. Alternatively one can consider only matrices with entries in an algebraically closed field, such as ⁠ℂ,⁠ from the outset.

Matrices whose entries are polynomials, and more generally, matrices with entries in a ring R are widely used in mathematics. Rings are a more general notion than fields in that a division operation need not exist. The very same addition and multiplication operations of matrices extend to this setting, too. The set M(n, R) (also denoted Mn(R)) of all square n-by-n matrices over R is a ring called matrix ring, isomorphic to the endomorphism ring of the left R-module Rn. If the ring R is commutative, that is, its multiplication is commutative, then the ring M(n, R) is also an associative algebra over R. The determinant of square matrices over a commutative ring R can still be defined using the Leibniz formula; such a matrix is invertible if and only if its determinant is invertible in R, generalizing the situation over a field F, where every nonzero element is invertible. Matrices over superrings are called supermatrices.

Matrices do not always have all their entries in the same ring – or even in any ring at all. One special but common case is block matrices, which may be considered as matrices whose entries themselves are matrices. The entries need not be square matrices, and thus need not be members of any ring; but in order to multiply them, their sizes must fulfill certain conditions: each pair of submatrices that are multiplied in forming the overall product must have compatible sizes.

### Relationship to linear maps

Linear maps ℝn→ℝm are equivalent to m-by-n matrices, as described above. More generally, any linear map f : V → W between finite-dimensional vector spaces can be described by a matrix A = (aij), after choosing bases v1, ..., vn of V, and w1, ..., wm of W (so n is the dimension of V and m is the dimension of W), which is such that

f(𝐯j)=∑i=1mai,j𝐰ifor j=1,…,n.

In other words, column j of A expresses the image of vj in terms of the basis vectors wi of W; thus this relation uniquely determines the entries of the matrix A. The matrix depends on the choice of the bases: different choices of bases give rise to different, but equivalent matrices. Many of the above concrete notions can be reinterpreted in this light, for example, the transpose matrix AT describes the transpose of the linear map given by A, concerning the dual bases.

These properties can be restated more naturally: the category of matrices with entries in a field k with multiplication as composition is equivalent to the category of finite-dimensional vector spaces and linear maps over this field.

More generally, the set of m × n matrices can be used to represent the R-linear maps between the free modules Rm and Rn for an arbitrary ring R with unity. When n = m composition of these maps is possible, and this gives rise to the matrix ring of n × n matrices representing the endomorphism ring of Rn.

### Matrix groups

A group is a mathematical structure consisting of a set of objects together with a binary operation, that is, an operation combining any two objects to a third, subject to certain requirements. A group in which the objects are invertible ⁠n×n⁠ matrices and the group operation is matrix multiplication is called a matrix group of degree ⁠n⁠. Every such matrix group is a subgroup of (that is, a smaller group contained within) the group of all invertible ⁠n×n⁠ matrices, the general linear group of degree ⁠n⁠.

Any property of square matrices that is preserved under matrix products and inverses can be used to define a matrix group. For example, the set of all ⁠n×n⁠ matrices whose determinant is 1 form a group called the special linear group of degree ⁠n⁠. The set of orthogonal matrices, determined by the condition

𝐌T𝐌=𝐈,

form the orthogonal group. Every orthogonal matrix has determinant 1 or −1. Orthogonal matrices with determinant 1 form a group called the special orthogonal group.

Every finite group is isomorphic to a matrix group, as one can see by considering the regular representation of the symmetric group. General groups can be studied using matrix groups, which are comparatively well understood, using representation theory.

### Infinite matrices

It is also possible to consider matrices with infinitely many rows and columns. The basic operations introduced above are defined the same way in this case. Matrix multiplication, however, and all operations stemming therefrom are only meaningful when restricted to certain matrices, since the sum featuring in the above definition of the matrix product will contain an infinity of summands. An easy way to circumvent this issue is to restrict to finitary matrices all of whose rows (or columns) contain only finitely many nonzero terms. As in the finite case (see above), where matrices describe linear maps, infinite matrices can be used to describe operators on Hilbert spaces, where convergence and continuity questions arise. However, the explicit point of view of matrices tends to obfuscate the matter, and the abstract and more powerful tools of functional analysis are used instead, by relating matrices to linear maps (as in the finite case above), but imposing additional convergence and continuity constraints.

### Empty matrix

An empty matrix is a matrix in which the number of rows or columns (or both) is zero. Empty matrices can be a useful base case for certain recursive constructions, and can help to deal with maps involving the zero vector space. For example, if A is a 3 × 0 matrix and B is a 0 × 3 matrix, then AB is the 3 × 3 zero matrix corresponding to the null map from a 3-dimensional space V to itself, while BA is a 0 × 0 matrix. There is no common notation for empty matrices, but most computer algebra systems allow creating and computing with them. The determinant of the 0 × 0 matrix is conventionally defined to be 1, consistent with the empty product occurring in the Leibniz formula for the determinant. This value is also needed for consistency with the 2 × 2 case of the Desnanot–Jacobi identity relating determinants to the determinants of smaller matrices.

### Matrices with entries in a semiring

A semiring is similar to a ring, but elements need not have additive inverses, therefore one cannot do subtraction freely there. The definition of addition and multiplication of matrices with entries in a ring applies to matrices with entries in a semiring without modification. Matrices of fixed size with entries in a semiring form a commutative monoid Mat⁡(m,n;R) under addition. Square matrices of fixed size with entries in a semiring form a semiring Mat⁡(n;R) under addition and multiplication.

The determinant of an n × n square matrix M with entries in a commutative semiring R cannot be defined in general because the definition would involve additive inverses of semiring elements. What plays its role instead is the pair of positive and negative determinants

det+M=∑σ∈Alt⁡(n)M1σ(1)⋯Mnσ(n)

det−M=∑σ∈Sym⁡(n)∖Alt⁡(n)M1σ(1)⋯Mnσ(n)

where the sums are taken over even permutations and odd permutations, respectively.

### Matrices with entries in a category

Matrices and their multiplication can be defined with entries objects of a category equipped with a "tensor product" similar to multiplication in a ring, having coproducts similar to addition in a ring, in that the former is distributive over the latter. However, the multiplication thus defined may be only associative in a sense weaker than usual. These are part of a bigger structure called the bicategory of matrices. The complete description of the above summary for interested readers follows.

Let (𝒞,⊗,I) be a monoidal category satisfying the following two conditions:

All (small) coproducts exist; in particular, let ∅ be an initial object.

The functor ⊗ is distributive over coproducts; i.e., for all object X and a family of objects (Yi)i∈I in 𝒞, the canonical 𝒞-morphisms ∐i∈I(X⊗Yi)→X⊗∐i∈IYi∐i∈I(Yi⊗X)→(∐i∈IYi)⊗X are isomorphisms. In particular, the canonical morphisms ∅→X⊗∅ and ∅→∅⊗X are isomorphisms.

Then, the bicategory of 𝒞-matrices Mat⁡(𝒞) is as follows:

The objects are the sets.

A 1-morphism M:A→B is a map M:A×B→Ob⁡(𝒞); this is just a matrix over 𝒞.

The composition of 1-morphisms M:A→B and N:B→C, which can be understood as matrix multiplication, is (N∘M)(a,c)=∐b∈BM(a,b)⊗N(b,c).

The identity 1-morphism on A is idA(a,b)={Ia=b∅a≠b.

A 2-morphism between 1-morphisms M,N:A→B is a family of 𝒞-morphisms (fab:M(a,b)→N(a,b))(a,b)∈A×B. The definition of vertical and horizontal composition of 2-morphisms is natural: the vertical composition is componentwise composition of 𝒞-morphisms; the horizontal composition is that derived from the functoriality of ⊗ and the universal property of coproducts.

In general, the bicategory of matrices need not be a strict 2-category. For example, the composition of 1-morphisms may not be associative in the usual strict sense, but only up to coherent isomorphism.

## Applications

There are numerous applications of matrices, both in mathematics and other sciences. Some of them merely take advantage of the compact representation of a set of numbers in a matrix. For example, text mining and automated thesaurus compilation makes use of document-term matrices such as tf-idf to track frequencies of certain words in several documents.

Complex numbers can be represented by particular real 2-by-2 matrices via

a+ib↔[a−bba],

under which addition and multiplication of complex numbers and matrices correspond to each other. For example, 2-by-2 rotation matrices represent the multiplication with some complex number of absolute value 1, as above. A similar interpretation is possible for quaternions and Clifford algebras in general.

In game theory and economics, the payoff matrix encodes the payoff for two players, depending on which out of a given (finite) set of strategies the players choose. The expected outcome of the game, when both players play mixed strategies, is obtained by multiplying this matrix on both sides by vectors representing the strategies. The minimax theorem central to game theory is closely related to the duality theory of linear programs, which are often formulated in terms of matrix-vector products.

Early encryption techniques such as the Hill cipher also used matrices. However, due to the linear nature of matrices, these codes are comparatively easy to break. Computer graphics uses matrices to represent objects; to calculate transformations of objects using affine rotation matrices to accomplish tasks such as projecting a three-dimensional object onto a two-dimensional screen, corresponding to a theoretical camera observation; and to apply image convolutions such as sharpening, blurring, edge detection, and more. Matrices over a polynomial ring are important in the study of control theory.

Chemistry makes use of matrices in various ways, particularly since the use of quantum theory to discuss molecular bonding and spectroscopy. Examples are the overlap matrix and the Fock matrix used in solving the Roothaan equations to obtain the molecular orbitals of the Hartree–Fock method.

### Graph theory

The adjacency matrix of a finite graph is a basic notion of graph theory. It records which vertices of the graph are connected by an edge. Matrices containing just two different values (1 and 0 meaning for example "yes" and "no", respectively) are called logical matrices. The distance (or cost) matrix contains information about the distances of the edges. These concepts can be applied to websites connected by hyperlinks, or cities connected by roads etc., in which case (unless the connection network is extremely dense) the matrices tend to be sparse, that is, contain few nonzero entries. Therefore, specifically tailored matrix algorithms can be used in network theory.

### Analysis and geometry

The Hessian matrix of a differentiable function f:ℝn→ℝ consists of the second derivatives of ƒ concerning the several coordinate directions, that is,

H(f)=[∂2f∂xi∂xj].

It encodes information about the local growth behavior of the function: given a critical point x = (x1, ..., xn), that is, a point where the first partial derivatives ∂f/∂xi of f vanish, the function has a local minimum if the Hessian matrix is positive definite. Quadratic programming can be used to find global minima or maxima of quadratic functions closely related to the ones attached to matrices (see above).

Another matrix frequently used in geometrical situations is the Jacobi matrix of a differentiable map ⁠f:ℝn→ℝm⁠. If f1, ..., fm denote the components of f, then the Jacobi matrix is defined as

J(f)=[∂fi∂xj]1≤i≤m,1≤j≤n.

If n > m, and if the rank of the Jacobi matrix attains its maximal value m, f is locally invertible at that point, by the implicit function theorem.

Partial differential equations can be classified by considering the matrix of coefficients of the highest-order differential operators of the equation. For elliptic partial differential equations this matrix is positive definite, which has a decisive influence on the set of possible solutions of the equation in question.

The finite element method is an important numerical method to solve partial differential equations, widely applied in simulating complex physical systems. It attempts to approximate the solution to some equation by piecewise linear functions, where the pieces are chosen concerning a sufficiently fine grid, which in turn can be recast as a matrix equation.

### Probability theory and statistics

Stochastic matrices are square matrices whose rows are probability vectors, that is, whose entries are non-negative and sum up to one. Stochastic matrices are used to define Markov chains with finitely many states. A row of the stochastic matrix gives the probability distribution for the next position of some particle currently in the state that corresponds to the row. Properties of the Markov chain—like absorbing states, that is, states that any particle attains eventually—can be read off the eigenvectors of the transition matrices.

Statistics also makes use of matrices in many different forms. Descriptive statistics is concerned with describing data sets, which can often be represented as data matrices, which may then be subjected to dimensionality reduction techniques. The covariance matrix encodes the mutual variance of several random variables. Another technique using matrices are linear least squares, a method that approximates a finite set of pairs (x1, y1), (x2, y2), ..., (xN, yN), by a linear function

yi≈axi+b,i=1,…,N

which can be formulated in terms of matrices, related to the singular value decomposition of matrices.

Random matrices are matrices whose entries are random numbers, subject to suitable probability distributions, such as matrix normal distribution. Beyond probability theory, they are applied in domains ranging from number theory to physics.

### Quantum mechanics and particle physics

The first model of quantum mechanics (Heisenberg, 1925) used infinite-dimensional matrices to define the operators that took over the role of variables like position, momentum and energy from classical physics. (This is sometimes referred to as matrix mechanics.) Matrices, both finite and infinite-dimensional, have since been employed for many purposes in quantum mechanics. One particular example is the density matrix, a tool used in calculating the probabilities of the outcomes of measurements performed on physical systems.

Linear transformations and the associated symmetries play a key role in modern physics. For example, elementary particles in quantum field theory are classified as representations of the Lorentz group of special relativity and, more specifically, by their behavior under the spin group. Concrete representations involving the Pauli matrices and more general gamma matrices are an integral part of the physical description of fermions, which behave as spinors. For the three lightest quarks, there is a group-theoretical representation involving the special unitary group SU(3); for their calculations, physicists use a convenient matrix representation known as the Gell-Mann matrices, which are also used for the SU(3) gauge group that forms the basis of the modern description of strong nuclear interactions, quantum chromodynamics. The Cabibbo–Kobayashi–Maskawa matrix, in turn, expresses the fact that the basic quark states that are important for weak interactions are not the same as, but linearly related to the basic quark states that define particles with specific and distinct masses.

Another matrix serves as a key tool for describing the scattering experiments that form the cornerstone of experimental particle physics: Collision reactions such as occur in particle accelerators, where non-interacting particles head towards each other and collide in a small interaction zone, with a new set of non-interacting particles as the result, can be described as the scalar product of outgoing particle states and a linear combination of ingoing particle states. The linear combination is given by a matrix known as the S-matrix, which encodes all information about the possible interactions between particles.

### Normal modes

A general application of matrices in physics is the description of linearly coupled harmonic systems. The equations of motion of such systems can be described in matrix form, with a mass matrix multiplying a generalized velocity to give the kinetic term, and a force matrix multiplying a displacement vector to characterize the interactions. The best way to obtain solutions is to determine the system's eigenvectors, its normal modes, by diagonalizing the matrix equation. Techniques like this are crucial when it comes to the internal dynamics of molecules: the internal vibrations of systems consisting of mutually bound component atoms. They are also needed for describing mechanical vibrations, and oscillations in electrical circuits.

### Geometrical optics

Geometrical optics provides further matrix applications. In this approximative theory, the wave nature of light is neglected. The result is a model in which light rays are indeed geometrical rays. If the deflection of light rays by optical elements is small, the action of a lens or reflective element on a given light ray can be expressed as multiplication of a two-component vector with a two-by-two matrix called ray transfer matrix analysis: the vector's components are the light ray's slope and its distance from the optical axis, while the matrix encodes the properties of the optical element. There are two kinds of matrices, viz. a refraction matrix describing the refraction at a lens surface, and a translation matrix, describing the translation of the plane of reference to the next refracting surface, where another refraction matrix applies.

The optical system, consisting of a combination of lenses and reflective elements, is simply described by the matrix resulting from the product of the components' matrices.

The Jones calculus models the polarization of a light source as a 2×2 vector, and the effects of optical filters on this polarization vector as a matrix.

### Electronics

Electronic circuits that are composed of linear components (such as resistors, inductors and capacitors) obey Kirchhoff's circuit laws, which leads to a system of linear equations, which can be described with a matrix equation that relates the source currents and voltages to the resultant currents and voltages at each point in the circuit, and where the matrix entries are determined by the circuit.

## History

Matrices have a long history of application in solving linear equations but they were known as arrays until the 1800s. The Chinese text The Nine Chapters on the Mathematical Art written in the 10th–2nd century BCE is the first example of the use of array methods to solve simultaneous equations. In 1545 Italian mathematician Gerolamo Cardano introduced the method to Europe when he published Ars Magna. The Japanese mathematician Seki used the same array methods to solve simultaneous equations in 1683. The Dutch mathematician Jan de Witt represented transformations using arrays in his 1659 book Elements of Curves (1659). Between 1700 and 1710 Gottfried Wilhelm Leibniz publicized the use of arrays for recording information or solutions and experimented with over 50 different systems of arrays. Cramer presented his rule in 1750.

This use of the term matrix in mathematics (an English word for "womb" in the 19th century, from Latin, as well as a jargon word in printing, in biology and in geology) was coined by James Joseph Sylvester in 1850, who understood a matrix as an object giving rise to several determinants today called minors, that is to say, determinants of smaller matrices that derive from the original one by removing columns and rows. In an 1851 paper, Sylvester explains:

I have in previous papers defined a "Matrix" as a rectangular array of terms, out of which different systems of determinants may be engendered from the womb of a common parent.

Arthur Cayley published a treatise on geometric transformations using matrices that were not rotated versions of the coefficients being investigated as had previously been done. Instead, he defined operations such as addition, subtraction, multiplication, and division as transformations of those matrices and showed the associative and distributive properties held. Cayley investigated and demonstrated the non-commutative property of matrix multiplication as well as the commutative property of matrix addition. Early matrix theory had limited the use of arrays almost exclusively to determinants and Cayley's abstract matrix operations were revolutionary. He was instrumental in proposing a matrix concept independent of equation systems. In 1858, Cayley published his A memoir on the theory of matrices in which he proposed and demonstrated the Cayley–Hamilton theorem.

The English mathematician Cuthbert Edmund Cullis was the first to use modern bracket notation for matrices in 1913 and he simultaneously demonstrated the first significant use of the notation A = [ai,j] to represent a matrix where ai,j refers to the ith row and the jth column.

The modern study of determinants sprang from several sources. Number-theoretical problems led Gauss to relate coefficients of quadratic forms, that is, expressions such as x2 + xy − 2y2, and linear maps in three dimensions to matrices. Eisenstein further developed these notions, including the remark that, in modern parlance, matrix products are non-commutative. Cauchy was the first to prove general statements about determinants, using as the definition of the determinant of a matrix A = [ai,j] the following: replace the powers ajk by aj,k in the polynomial

a1a2⋯an∏i<j(aj−ai),

where ∏ denotes the product of the indicated terms. He also showed, in 1829, that the eigenvalues of symmetric matrices are real. Jacobi studied "functional determinants"—later called Jacobi determinants by Sylvester—which can be used to describe geometric transformations at a local (or infinitesimal) level, see above. Kronecker's Vorlesungen über die Theorie der Determinanten and Weierstrass's Zur Determinantentheorie, both published in 1903, first treated determinants axiomatically, as opposed to previous more concrete approaches such as the mentioned formula of Cauchy. At that point, determinants were firmly established.

Many theorems were first established for small matrices only, for example, the Cayley–Hamilton theorem was proved for 2 × 2 matrices by Cayley in the aforementioned memoir, and by Hamilton for 4 × 4 matrices. Frobenius, working on bilinear forms, generalized the theorem to all dimensions (1898). Also at the end of the 19th century, the Gauss–Jordan elimination (generalizing a special case now known as Gauss elimination) was established by Wilhelm Jordan. In the early 20th century, matrices attained a central role in linear algebra, partially due to their use in the classification of the hypercomplex number systems of the previous century.

The inception of matrix mechanics by Heisenberg, Born and Jordan led to studying matrices with infinitely many rows and columns. Later, von Neumann carried out the mathematical formulation of quantum mechanics, by further developing functional analytic notions such as linear operators on Hilbert spaces, which, very roughly speaking, correspond to Euclidean space, but with an infinity of independent directions.

### Other historical usages of the word "matrix" in mathematics

The word has been used in unusual ways by at least two authors of historical importance.

Bertrand Russell and Alfred North Whitehead in their Principia Mathematica (1910–1913) use the word "matrix" in the context of their axiom of reducibility. They proposed this axiom as a means to reduce any function to one of lower type, successively, so that at the "bottom" (0 order) the function is identical to its extension:

Let us give the name of matrix to any function, of however many variables, that does not involve any apparent variables. Then, any possible function other than a matrix derives from a matrix using generalization, that is, by considering the proposition that the function in question is true with all possible values or with some value of one of the arguments, the other argument or arguments remaining undetermined.

For example, a function Φ(x, y) of two variables x and y can be reduced to a collection of functions of a single variable, such as y, by "considering" the function for all possible values of "individuals" ai substituted in place of a variable x. And then the resulting collection of functions of the single variable y, that is, ∀ai: Φ(ai, y), can be reduced to a "matrix" of values by "considering" the function for all possible values of "individuals" bi substituted in place of variable y:

∀bj∀ai:ϕ(ai,bj).

Alfred Tarski in his 1941 Introduction to Logic used the word "matrix" synonymously with the notion of truth table as used in mathematical logic.
