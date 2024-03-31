package serializer.xml;//Based on example code from: https://examples.javacodegeeks.com/core-java/xml/parsers/documentbuilderfactory/create-xml-file-in-java-using-dom-parser-example/

import java.io.File;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;
import javax.xml.transform.*;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;
import org.w3c.dom.Document;
import org.w3c.dom.Element;
import serializer.peg.MutantsLog;
import serializer.peg.Pair;
import serializer.peg.PegNode;

public class XMLGenerator {

    public static final String dirPath = System.getProperty("user.dir");

    // Map method signatures to their associated <subject> elements
    private Map<String, Element> methodToSubject =  new HashMap<>();

    private Element subjects;
    private Document document;
    private Transformer transformer;

    public XMLGenerator() {
        DocumentBuilderFactory dbf = DocumentBuilderFactory.newInstance();
        DocumentBuilder documentBuilder;
        try {
            documentBuilder = dbf.newDocumentBuilder();
        } catch (ParserConfigurationException pce) {
            throw new RuntimeException("Couldn't create document builder");
        }
        document = documentBuilder.newDocument();

        //setup transofrmer
        TransformerFactory transformerFactory = TransformerFactory.newInstance();
        try {
            transformer = transformerFactory.newTransformer();
        } catch (TransformerConfigurationException tce) {
            tce.printStackTrace();
            throw new RuntimeException("Couldn't create transformer");
        }
        // Pretty print with indents
        transformer.setOutputProperty(OutputKeys.INDENT, "yes");
        // Number of spaces
        // transformer.setOutputProperty("{http://xml.apache.org/xslt}indent-amount", "2");

        // No longer needed since subjects is cut out of spec
        subjects = document.createElement("subjects");
        document.appendChild(subjects);
    }

    /**
     * This method creates a new {@code <subject>} element and stores it in a map associated with key {@code method}
     * @param sourceFile name of source file containing the original method
     * @param methodString fully qualified method signature Class@methodName(T1,T2,T3)
     * @param pegId: the peg id, produced by serialization, for the original method
     */
    public void addSubject(String sourceFile, String methodString, int pegId) {
        final Element subject = document.createElement("subject");
        subject.setAttribute("sourcefile", sourceFile);
        subject.setAttribute("method", methodString);
        subjects.appendChild(subject);

        final Element origPid = document.createElement("pid");
        origPid.appendChild(document.createTextNode(Integer.valueOf(pegId).toString()));
        subject.appendChild(origPid);

        methodToSubject.put(methodString, subject);
    }

    /**
     * This method creates a new {@code <subject>} element and stores it in a map associated with key {@code method}
     * and adds all the associated mutants in {@code log} to the created subject.
     * @param sourceFile name of source file containing the original method
     * @param methodString fully qualified method signature Class@methodName(T1,T2,T3)
     * @param pegId: the peg id, produced by serialization, for the original method
     * @param log a list of log entries of mutants that have been serialized that we should add to this subject field
     */
    public void addSubject(String sourceFile, String methodString, int pegId, List<MutantsLog.Row> log) {
        addSubject(sourceFile, methodString, pegId);
        for (MutantsLog.Row row : log) {
            addMutant(methodString, row.id, row.pegId);
        }
    }

    /**
     *
     * @param methodString fully qualified method signature Class@methodName(T1,T2,T3)
     * @param mutantId the mutant's mutant id, generated by Major, as found in the {@code mutants.log} file
     * @param pegId the mutant's peg id generated by Cornelius during serialization
     */
    public void addMutant(String methodString, String mutantId, int pegId) {
        if (methodToSubject.containsKey(methodString)) {

            Element subject = methodToSubject.get(methodString); // does this actually get and modifiy the element in the map?

            Element mutant = document.createElement("mutant");
            mutant.setAttribute("mid", mutantId);
            mutant.setAttribute("pid", String.valueOf(pegId));
            subject.appendChild(mutant);
        }
        else {
            throw new IllegalArgumentException("Map does not contain associated subject for mutant");
        }
    }

    public boolean hasSubject() {
        return subjects.getElementsByTagName("subject").getLength() != 0;
    }

    public int numSubjects() {
        return subjects.getElementsByTagName("subject").getLength();
    }

    /**
     * Add the {@code <id_table>} element to the xml doc
     * @param idTable map from id to pegnode
     */
    public void addIdTable(Map<Integer, PegNode> idTable) {
        Element table = document.createElement("id_table");
        subjects.appendChild(table);
        final List<Integer> keys = new ArrayList<>(idTable.keySet());
        keys.sort(null);
        for (Integer id : keys) {
            final PegNode p = idTable.get(id);
            Element dedupEntry = document.createElement("dedup_entry");
            table.appendChild(dedupEntry);
            dedupEntry.setAttribute("id", id.toString());
            dedupEntry.setAttribute("peg", p.toString());
        }
    }

    /**
     * Add a list of equivalences to the xml doc
     * @param equivs a list of ids to be marked as equivalent
     */
    public void addEquivalences(List<Pair<Integer, Integer>> equivs) {
        Element table = document.createElement("node_equivalences");
        subjects.appendChild(table);
        for (Pair<Integer, Integer> equiv : equivs) {
            Element equivElement = document.createElement("node_equivalence");

            Element first = document.createElement("first");
            Element second = document.createElement("second");
            first.setTextContent(equiv.fst.toString());
            second.setTextContent(equiv.snd.toString());
            equivElement.appendChild(first);
            equivElement.appendChild(second);

            table.appendChild(equivElement);
        }
    }

    /**
     * Print the XML document to console
     */
    public void writeToConsole() {
        // create the xml file
        DOMSource domSource = new DOMSource(document);
        StreamResult result = new StreamResult(System.out);
        try {
            transformer.transform(domSource, result);
        } catch (TransformerException e) {
            e.printStackTrace();
        }
    }

    /**
     * Write the XML file we've built to disk as {@code filename}
     * @param filename  name of file to write to
     * @throws TransformerException  when an unrecoverable error occurs durring {@code transformer.transform}; see
     * {@link javax.xml.transform.Transformer#transform(Source, Result)}
     */
    public void writeToFile(String filename) throws TransformerException {
        DOMSource domSource = new DOMSource(document);
        StreamResult streamResult = new StreamResult(new File(dirPath + "/" + filename));
        transformer.transform(domSource, streamResult);
    }
}